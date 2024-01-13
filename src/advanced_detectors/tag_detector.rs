// Detect a html tag 
// ------------------
// Properties:
// - tag: The tag name (if not specified, any tag will be detected)
// - has_attributes: Whether the tag has attributes or not
//   - None: Any tag will be detected
//   - Some(true): Only tags with attributes will be detected
//   - Some(false): Only tags without attributes will be detected
// - attributes: The keys the tag must have (if not specified, any tag will be detected)
// - is_closing: Whether the tag is closing or not (starting with </)
//   - None: Any tag will be detected
//   - Some(true): Only closing tags will be detected
//   - Some(false): Only non-closing tags will be detected
// - is_self_closing: Whether the tag is self-closing or not (ending with />)
//   - None: Any tag will be detected
//   - Some(true): Only self-closing tags will be detected
//   - Some(false): Only non-self-closing tags will be detected
// - is_opening: Whether the tag is opening or not (not closing or self-closing)
//   - None: Any tag will be detected
//   - Some(true): Only opening tags will be detected
//   - Some(false): Only non-opening tags will be detected

use regex::Regex;

use crate::detectors::word_detector::{identifier_detector, whitespace_detector};
use crate::detectors::{
    scope_detector::ScopeDetector,
    property_detector::PropertyDetector,
    word_detector::WordDetector
};

use crate::base::*;
use crate::types::{Queue, Dict};

#[derive(Debug, Clone)]
pub struct TagDetector {
    pub tag: Option<Regex>,
    pub has_attributes: Option<bool>,
    pub is_closing: Option<bool>,
    pub is_self_closing: Option<bool>,
    pub is_opening: Option<bool>
}

impl TagDetector {
    pub fn new(
        tag: Option<String>,
        has_attributes: Option<bool>,
        is_closing: Option<bool>,
        is_self_closing: Option<bool>,
        is_opening: Option<bool>
    ) -> Self {
        Self {
            tag: match tag {
                Some(tag) => Some(Regex::new(tag.as_str()).ok().unwrap()),
                None => None
            },
            has_attributes,
            is_closing,
            is_self_closing,
            is_opening
        }
    }

    pub fn new_regex(
        tag: Option<Regex>,
        has_attributes: Option<bool>,
        is_closing: Option<bool>,
        is_self_closing: Option<bool>,
        is_opening: Option<bool>
    ) -> Self {
        Self {
            tag,
            has_attributes,
            is_closing,
            is_self_closing,
            is_opening
        }
    }
}

impl Detectable for TagDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        let start_detector = Detector::WordDetector(WordDetector::new(Some("<".to_string()), None, None));

        let end_detector = Detector::WordDetector(WordDetector::new(Some(">".to_string()), None, None));

        let scope_detector = Detector::ScopeDetector(ScopeDetector::new(
            Box::new(start_detector),
            Box::new(end_detector)
        ));

        let (_, _, inner_result) = queue.consume(&scope_detector);

        if self.is_closing.unwrap_or(false) && self.is_opening.unwrap_or(false) {
            return None;
        }

        if self.is_closing.unwrap_or(false) && self.is_self_closing.unwrap_or(false) {
            return None;
        }

        match inner_result {
            Some(inner_result) => {
                let mut queue = inner_result.content.unwrap();

                let closing;

                // Check if the tag is closing
                if queue[0] == '/' {
                    closing = true;

                    if self.is_closing.unwrap_or(true) == false {
                        return None;
                    }
                    if self.is_opening.unwrap_or(false) == true {
                        return None;
                    }

                    queue.remove(0);
                } else {
                    closing = false;

                    if self.is_opening.unwrap_or(true) == false {
                        return None;
                    }
                    if self.is_closing.unwrap_or(false) == true {
                        return None;
                    }
                }

                let indentifier_detector = Detector::WordDetector(identifier_detector());
                let whitespace_detector = Detector::WordDetector(whitespace_detector());

                // Consume whitespace
                queue.consume(&whitespace_detector);

                // Consume tag name
                let (matched, tag, _) = queue.consume(&indentifier_detector);

                if !matched {
                    return None;
                }

                let tag = tag.unwrap();

                // Check if the tag is the correct tag
                match &self.tag {
                    Some(tag_name) => {
                        if !tag_name.is_match(tag.as_str()) {
                            return None;
                        }
                    },
                    None => {}
                }

                // While there are attributes, consume them
                let attribute_detector = Detector::PropertyDetector(PropertyDetector::new(Some(false), Some(true)));

                let mut attributes: Dict = Dict::new();

                loop {
                    let (matched, _, result) = queue.consume(&attribute_detector);

                    if !matched {
                        break;
                    }

                    match result {
                        Some(result) => {
                            match result.properties {
                                Some(properties) => {
                                    // Get key and value
                                    let key = properties.get("key");
                                    let value = properties.get("value");

                                    // Check if the attribute is already defined
                                    if attributes.has(key.to_str().unwrap().as_str()) {
                                        return None;
                                    }

                                    // Add the attribute to the list
                                    attributes.set(
                                        key.to_str().unwrap().as_str(),
                                        value
                                    );
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                }

                if !attributes.empty() {
                    if self.has_attributes.unwrap_or(true) == false || closing {
                        return None;
                    }
                } else {
                    if self.has_attributes.unwrap_or(false) == true {
                        return None;
                    }
                }

                // Consuming whitespace
                queue.consume(&whitespace_detector);

                let self_closing: bool;

                // Check if the tag is self-closing
                if queue.len() == 0 || queue[0] != '/' {
                    self_closing = false;

                    if self.is_self_closing.unwrap_or(false) == true {
                        return None;
                    }
                } else {
                    self_closing = true;

                    if self.is_self_closing.unwrap_or(true) == false {
                        return None;
                    }

                    if closing {
                        return None;
                    }

                    queue.remove(0);
                }

                // If the queue is not empty, the tag is invalid
                if !queue.is_empty() {
                    return None;
                }

                // Return the result
                let mut result = Result::new(Detector::TagDetector(self.clone()), None, None, None);

                result.properties = Some(
                    Dict::from(
                        vec![
                            ("tag".to_string(), &tag),
                            ("attributes".to_string(), &attributes),
                            ("closing".to_string(), &closing),
                            ("self_closing".to_string(), &self_closing),
                            ("opening".to_string(), &!closing)
                        ]
                    )
                );

                Some(result)
            },
            None => None
        }
    }
}

impl PartialEq for TagDetector {
    fn eq(&self, other: &Self) -> bool {
        (
            self.tag.clone().unwrap_or(Regex::new(r"").ok().unwrap()).as_str() == 
            other.tag.clone().unwrap_or(Regex::new(r"").ok().unwrap()).as_str()
        ) &&
        self.has_attributes == other.has_attributes &&
        self.is_closing == other.is_closing &&
        self.is_self_closing == other.is_self_closing &&
        self.is_opening == other.is_opening
    }
}

// Tests
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_detector() {
        let queue_opening = Queue::from_string("<div class=\"test\">".to_string());

        test_any(&mut queue_opening.clone(), true);

        test_tag_name(&mut queue_opening.clone(), Some("div"), true);
        test_tag_name(&mut queue_opening.clone(), Some("span"), false);
        test_tag_name(&mut queue_opening.clone(), None, true);

        test_has_attributes(&mut queue_opening.clone(), Some(true), true);
        test_has_attributes(&mut queue_opening.clone(), Some(false), false);
        test_has_attributes(&mut queue_opening.clone(), None, true);

        test_is_closing(&mut queue_opening.clone(), Some(true), false);
        test_is_closing(&mut queue_opening.clone(), Some(false), true);
        test_is_closing(&mut queue_opening.clone(), None, true);

        test_is_self_closing(&mut queue_opening.clone(), Some(true), false);
        test_is_self_closing(&mut queue_opening.clone(), Some(false), true);
        test_is_self_closing(&mut queue_opening.clone(), None, true);

        test_is_opening(&mut queue_opening.clone(), Some(true), true);
        test_is_opening(&mut queue_opening.clone(), Some(false), false);
        test_is_opening(&mut queue_opening.clone(), None, true);

        test_open_and_close(&mut queue_opening.clone());
        test_self_closing_and_close(&mut queue_opening.clone());

        let queue_closing = Queue::from_string("</div>".to_string());

        test_any(&mut queue_closing.clone(), true);

        test_tag_name(&mut queue_closing.clone(), Some("div"), true);
        test_tag_name(&mut queue_closing.clone(), Some("span"), false);
        test_tag_name(&mut queue_closing.clone(), None, true);

        test_has_attributes(&mut queue_closing.clone(), Some(true), false);
        test_has_attributes(&mut queue_closing.clone(), Some(false), true);
        test_has_attributes(&mut queue_closing.clone(), None, true);

        test_is_closing(&mut queue_closing.clone(), Some(true), true);
        test_is_closing(&mut queue_closing.clone(), Some(false), false);
        test_is_closing(&mut queue_closing.clone(), None, true);

        test_is_self_closing(&mut queue_closing.clone(), Some(true), false);
        test_is_self_closing(&mut queue_closing.clone(), Some(false), true);
        test_is_self_closing(&mut queue_closing.clone(), None, true);

        test_is_opening(&mut queue_closing.clone(), Some(true), false);
        test_is_opening(&mut queue_closing.clone(), Some(false), true);
        test_is_opening(&mut queue_closing.clone(), None, true);

        test_open_and_close(&mut queue_closing.clone());
        test_self_closing_and_close(&mut queue_closing.clone());

        let queue_closing_attributes = Queue::from_string("</div class=\"test\">".to_string());

        // This should always fail
        test_any(&mut queue_closing_attributes.clone(), false);

        test_tag_name(&mut queue_closing_attributes.clone(), Some("div"), false);
        test_tag_name(&mut queue_closing_attributes.clone(), Some("span"), false);
        test_tag_name(&mut queue_closing_attributes.clone(), None, false);

        test_has_attributes(&mut queue_closing_attributes.clone(), Some(true), false);
        test_has_attributes(&mut queue_closing_attributes.clone(), Some(false), false);
        test_has_attributes(&mut queue_closing_attributes.clone(), None, false);

        test_is_closing(&mut queue_closing_attributes.clone(), Some(true), false);
        test_is_closing(&mut queue_closing_attributes.clone(), Some(false), false);
        test_is_closing(&mut queue_closing_attributes.clone(), None, false);

        test_is_self_closing(&mut queue_closing_attributes.clone(), Some(true), false);
        test_is_self_closing(&mut queue_closing_attributes.clone(), Some(false), false);
        test_is_self_closing(&mut queue_closing_attributes.clone(), None, false);

        test_is_opening(&mut queue_closing_attributes.clone(), Some(true), false);
        test_is_opening(&mut queue_closing_attributes.clone(), Some(false), false);
        test_is_opening(&mut queue_closing_attributes.clone(), None, false);

        test_open_and_close(&mut queue_closing_attributes.clone());
        test_self_closing_and_close(&mut queue_closing_attributes.clone());

        let queue_self_closing = Queue::from_string("<div class=\"test\"/>".to_string());

        test_any(&mut queue_self_closing.clone(), true);

        test_tag_name(&mut queue_self_closing.clone(), Some("div"), true);
        test_tag_name(&mut queue_self_closing.clone(), Some("span"), false);
        test_tag_name(&mut queue_self_closing.clone(), None, true);

        test_has_attributes(&mut queue_self_closing.clone(), Some(true), true);
        test_has_attributes(&mut queue_self_closing.clone(), Some(false), false);
        test_has_attributes(&mut queue_self_closing.clone(), None, true);

        test_is_closing(&mut queue_self_closing.clone(), Some(true), false);
        test_is_closing(&mut queue_self_closing.clone(), Some(false), true);
        test_is_closing(&mut queue_self_closing.clone(), None, true);

        test_is_self_closing(&mut queue_self_closing.clone(), Some(true), true);
        test_is_self_closing(&mut queue_self_closing.clone(), Some(false), false);
        test_is_self_closing(&mut queue_self_closing.clone(), None, true);

        test_is_opening(&mut queue_self_closing.clone(), Some(true), true);
        test_is_opening(&mut queue_self_closing.clone(), Some(false), false);
        test_is_opening(&mut queue_self_closing.clone(), None, true);

        test_open_and_close(&mut queue_self_closing.clone());
        test_self_closing_and_close(&mut queue_self_closing.clone());

        let queue_opening_no_attributes = Queue::from_string("<div>".to_string());

        test_any(&mut queue_opening_no_attributes.clone(), true);

        test_tag_name(&mut queue_opening_no_attributes.clone(), Some("div"), true);
        test_tag_name(&mut queue_opening_no_attributes.clone(), Some("span"), false);
        test_tag_name(&mut queue_opening_no_attributes.clone(), None, true);

        test_has_attributes(&mut queue_opening_no_attributes.clone(), Some(true), false);
        test_has_attributes(&mut queue_opening_no_attributes.clone(), Some(false), true);
        test_has_attributes(&mut queue_opening_no_attributes.clone(), None, true);

        test_is_closing(&mut queue_opening_no_attributes.clone(), Some(true), false);
        test_is_closing(&mut queue_opening_no_attributes.clone(), Some(false), true);
        test_is_closing(&mut queue_opening_no_attributes.clone(), None, true);

        test_is_self_closing(&mut queue_opening_no_attributes.clone(), Some(true), false);
        test_is_self_closing(&mut queue_opening_no_attributes.clone(), Some(false), true);
        test_is_self_closing(&mut queue_opening_no_attributes.clone(), None, true);

        test_is_opening(&mut queue_opening_no_attributes.clone(), Some(true), true);
        test_is_opening(&mut queue_opening_no_attributes.clone(), Some(false), false);
        test_is_opening(&mut queue_opening_no_attributes.clone(), None, true);

        test_open_and_close(&mut queue_opening_no_attributes.clone());
        test_self_closing_and_close(&mut queue_opening_no_attributes.clone());

        let queue_opening_attributes_span = Queue::from_string("<span class=\"test\">".to_string());

        test_any(&mut queue_opening_attributes_span.clone(), true);

        test_tag_name(&mut queue_opening_attributes_span.clone(), Some("div"), false);
        test_tag_name(&mut queue_opening_attributes_span.clone(), Some("span"), true);
        test_tag_name(&mut queue_opening_attributes_span.clone(), None, true);

        test_has_attributes(&mut queue_opening_attributes_span.clone(), Some(true), true);
        test_has_attributes(&mut queue_opening_attributes_span.clone(), Some(false), false);
        test_has_attributes(&mut queue_opening_attributes_span.clone(), None, true);

        test_is_closing(&mut queue_opening_attributes_span.clone(), Some(true), false);
        test_is_closing(&mut queue_opening_attributes_span.clone(), Some(false), true);
        test_is_closing(&mut queue_opening_attributes_span.clone(), None, true);

        test_is_self_closing(&mut queue_opening_attributes_span.clone(), Some(true), false);
        test_is_self_closing(&mut queue_opening_attributes_span.clone(), Some(false), true);
        test_is_self_closing(&mut queue_opening_attributes_span.clone(), None, true);

        test_is_opening(&mut queue_opening_attributes_span.clone(), Some(true), true);
        test_is_opening(&mut queue_opening_attributes_span.clone(), Some(false), false);
        test_is_opening(&mut queue_opening_attributes_span.clone(), None, true);

        test_open_and_close(&mut queue_opening_attributes_span.clone());
        test_self_closing_and_close(&mut queue_opening_attributes_span.clone());
    }

    fn passed(name: &str, params: &str, queue: &str) {
        println!("{} ({}) passed for queue {}", name, params, queue);
    }

    fn test(name: &str, result: bool, queue: &mut Queue, wanted: bool, params: &str) {
        assert_eq!(
            result,
            wanted,
            "Expected the queue {} to {} match the tag detector with {}",
            queue.to_string(),
            if wanted { "" } else { "not " },
            params
        );

        passed(
            name,
            "None, None, None, None, None",
            &queue.to_string()
        )
    }

    fn test_any(queue: &mut Queue, wanted: bool) {
        let detector = Detector::TagDetector(TagDetector::new(None, None, None, None, None));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_any",
            matched,
            queue,
            wanted,
            "None, None, None, None, None"
        )
    }

    fn test_tag_name(queue: &mut Queue, tag_name: Option<&str>, wanted: bool) {
        let tag = match tag_name {
            Some(tag_name) => Some(tag_name.to_string()),
            None => None
        };

        let detector = Detector::TagDetector(
            TagDetector::new(
                tag.clone(), None, None, None, None
            ));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_tag_name",
            matched,
            queue,
            wanted,
            &format!("{:?}, None, None, None, None", tag)
        );
    }

    fn test_has_attributes(queue: &mut Queue, has_attributes: Option<bool>, wanted: bool) {
        let detector = Detector::TagDetector(TagDetector::new(None, has_attributes, None, None, None));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_has_attributes",
            matched,
            queue,
            wanted,
            &format!("None, {:?}, None, None, None", has_attributes)
        )
    }

    fn test_is_closing(queue: &mut Queue, is_closing: Option<bool>, wanted: bool) {
        let detector = Detector::TagDetector(TagDetector::new(None, None, is_closing, None, None));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_is_closing",
            matched,
            queue,
            wanted,
            &format!("None, None, {:?}, None, None", is_closing)
        )
    }

    fn test_is_self_closing(queue: &mut Queue, is_self_closing: Option<bool>, wanted: bool) {
        let detector = Detector::TagDetector(TagDetector::new(None, None, None, is_self_closing, None));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_is_self_closing",
            matched,
            queue,
            wanted,
            &format!("None, None, None, {:?}, None", is_self_closing)
        )
    }

    fn test_is_opening(queue: &mut Queue, is_opening: Option<bool>, wanted: bool) {
        let detector = Detector::TagDetector(TagDetector::new(None, None, None, None, is_opening));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_is_opening",
            matched,
            queue,
            wanted,
            &format!("None, None, None, None, {:?}", is_opening)
        )
    }

    fn test_open_and_close(queue: &mut Queue) {
        let detector = Detector::TagDetector(TagDetector::new(None, None, Some(true), None, Some(true)));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_open_and_close",
            matched,
            queue,
            false,
            "None, None, Some(true), None, Some(true)"
        )
    }

    fn test_self_closing_and_close(queue: &mut Queue) {
        let detector = Detector::TagDetector(TagDetector::new(None, None, Some(true), Some(true), None));

        let (matched, _, _) = queue.clone().consume(&detector);

        test(
            "test_self_closing_and_close",
            matched,
            queue,
            false,
            "None, None, Some(true), Some(true), None"
        )
    }
}