// Detect a Property of the style (regex) `key *= *"value"` or `key *: *'value'`
// --------------------------------------------------------------------------------

use crate::types::Dict;
use crate::{base::*, types::Queue};
use crate::detectors::word_detector::WordDetector;
use crate::detectors::scope_detector::ScopeDetector;

use super::word_detector::{whitespace_detector, identifier_detector};

#[derive(Debug, Clone)]
pub struct PropertyDetector {
    pub json: Option<bool>, // If true, we will only match json properties (key: value) otherwise we will match any property (key = value)
    pub html: Option<bool>, // If true, we will only match html properties (key="value") otherwise we will match any property (key = value)
}

impl PropertyDetector {
    pub fn new(json: Option<bool>, html: Option<bool>) -> Self {
        Self {
            json,
            html
        }
    }
}

fn detect_html(queue: &mut Queue) -> Option<Result> {
    // Consume whitespace
    queue.consume(&Detector::WordDetector(whitespace_detector()));

    // Consume key
    let (matched, key, _) = queue.consume(&Detector::WordDetector(identifier_detector()));

    if !matched {
        return None;
    }

    // Consume whitespace
    queue.consume(&Detector::WordDetector(whitespace_detector()));

    // Consume =
    let (matched, _, _) = queue.consume(&Detector::WordDetector(WordDetector::new(Some("=".to_string()), None, None)));

    if !matched {
        return None;
    }

    // Consume whitespace
    queue.consume(&Detector::WordDetector(whitespace_detector()));

    // Consume "" Scope
    let (matched, _, result) = queue.consume(&Detector::ScopeDetector(ScopeDetector::new(
        Box::new(Detector::WordDetector(WordDetector::new(Some("\"".to_string()), None, None))),
        Box::new(Detector::WordDetector(WordDetector::new(Some("\"".to_string()), None, None))),
    )));

    if !matched {
        return None;
    }

    match result {
        Some(result) => {
            match result.content {
                Some(content) => {
                    let value = content.to_string();

                    let mut result = Result::new(Detector::PropertyDetector(PropertyDetector::new(None, Some(true))), None, None, None);

                    result.properties = Some(
                        Dict::from(
                            vec![
                                ("key".to_string(), &key.unwrap()),
                                ("value".to_string(), &value)
                            ]
                        )
                    );

                    Some(result)
                },
                None => None
            }
        },
        None => None
    }
}

fn detect_json(queue: &mut Queue) -> Option<Result> {
    // Consume whitespace
    queue.consume(&Detector::WordDetector(whitespace_detector()));

    // Consume "" Scope
    let scope_detector = ScopeDetector::new(
        Box::new(Detector::WordDetector(WordDetector::new(Some("\"".to_string()), None, None))),
        Box::new(Detector::WordDetector(WordDetector::new(Some("\"".to_string()), None, None))),
    );

    let (matched, _, result) = queue.consume(&Detector::ScopeDetector(scope_detector.clone()));

    if !matched {
        return None;
    }

    // Set inner queue as key
    let key = result.unwrap().content.unwrap().to_string();

    // Consume whitespace
    queue.consume(&Detector::WordDetector(whitespace_detector()));

    // Consume :
    let (matched, _, _) = queue.consume(&Detector::WordDetector(WordDetector::new(Some(":".to_string()), None, None)));

    if !matched {
        return None;
    }

    // Consume whitespace
    queue.consume(&Detector::WordDetector(whitespace_detector()));

    // Consume "" Scope
    let (matched, _, result) = queue.consume(&Detector::ScopeDetector(ScopeDetector::new(
        Box::new(Detector::WordDetector(WordDetector::new(Some("\"".to_string()), None, None))),
        Box::new(Detector::WordDetector(WordDetector::new(Some("\"".to_string()), None, None))),
    )));

    if !matched {
        return None;
    }

    match result {
        Some(result) => {
            match result.content {
                Some(content) => {
                    let value = content.to_string();

                    let mut result = Result::new(Detector::PropertyDetector(PropertyDetector::new(Some(true), None)), None, None, None);

                    result.properties = Some(
                        Dict::from(
                            vec![
                                ("key".to_string(), &key),
                                ("value".to_string(), &value)
                            ]
                        )
                    );

                    Some(result)
                },
                None => None
            }
        },
        None => None
    }
}

fn detect_any(queue: &mut Queue) -> Option<Result> {
    let (matched_html, _, result_html) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(None, Some(true))));
    let (matched_json, _, result_json) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(Some(true), None)));

    if !(matched_html || matched_json) {
        return None;
    }

    match (&result_html, &result_json) {
        (Some(result_html), _) => Some(result_html.clone()),
        (_, Some(result_json)) => Some(result_json.clone()),
        (None, None) => None
    }
}

impl Detectable for PropertyDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        match (self.html, self.json) {
            (Some(true), Some(true)) => {
                detect_any(queue)
            },
            (Some(true), _) => {
                detect_html(queue)
            },
            (Some(false), _) => {
                detect_json(queue)
            },
            (_, Some(true)) => {
                detect_json(queue)
            },
            (_, Some(false)) => {
                detect_html(queue)
            },
            _ => {
                detect_any(queue)
            }
        }
    }
}

impl PartialEq for PropertyDetector {
    fn eq(&self, other: &Self) -> bool {
        self.html == other.html && self.json == other.json
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Queue, Value};

    use super::*;

    #[test]
    fn test_detect_html_correct() {
        let mut queue = Queue::from_string("key = \"value\"".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(None, Some(true))));

        assert_eq!(matched, true);

        match result {
            Some(result) => {
                match result.properties {
                    Some(properties) => {
                        assert_eq!(properties.get("key"), Value::String("key".to_string()));
                        assert_eq!(properties.get("value"), Value::String("value".to_string()));
                    },
                    None => assert_eq!(true, false)
                }
            },
            None => assert_eq!(true, false)
        }
    }

    #[test]
    fn test_detect_html_incorrect() {
        let mut queue = Queue::from_string("key = value".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(None, Some(true))));

        assert_eq!(matched, false);
        assert_eq!(result, None);

        let mut queue = Queue::from_string("key = \"value".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(None, Some(true))));

        assert_eq!(matched, false);
        assert_eq!(result, None);

        let mut queue = Queue::from_string("key = value\"".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(None, Some(true))));

        assert_eq!(matched, false);
        assert_eq!(result, None);

        // JSON
        let mut queue = Queue::from_string("\"key\" : \"value\"".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(None, Some(true))));

        assert_eq!(matched, false);
        assert_eq!(result, None);
    }   

    #[test]
    fn test_detect_json_correct() {
        let mut queue = Queue::from_string("\"key\" : \"value\"".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(Some(true), None)));

        assert_eq!(matched, true);

        match result {
            Some(result) => {
                match result.properties {
                    Some(properties) => {
                        assert_eq!(properties.get("key"), Value::String("key".to_string()));
                        assert_eq!(properties.get("value"), Value::String("value".to_string()));
                    },
                    None => assert_eq!(true, false)
                }
            },
            None => assert_eq!(true, false)
        }
    }

    #[test]
    fn test_detect_json_incorrect() {
        let mut queue = Queue::from_string("\"key\" : value".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(Some(true), None)));

        assert_eq!(matched, false);
        assert_eq!(result, None);

        let mut queue = Queue::from_string("\"key\" : \"value".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(Some(true), None)));

        assert_eq!(matched, false);
        assert_eq!(result, None);

        let mut queue = Queue::from_string("\"key\" : value\"".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(Some(true), None)));

        assert_eq!(matched, false);
        assert_eq!(result, None);

        // HTML
        let mut queue = Queue::from_string("key = \"value\"".to_string());

        let (matched, _, result) = queue.consume(&Detector::PropertyDetector(PropertyDetector::new(Some(true), None)));

        assert_eq!(matched, false);
        assert_eq!(result, None);
    }
}