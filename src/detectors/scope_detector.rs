// Scope Detector: Detect a scope between two other detectors
// ----------------------------------------------------------
//
// This detector is used to detect a scope between two other
// detectors whilst keeping track of the scope layers.
//
// ----------------------------------------------------------

use crate::{base::*, types::{Dict, Queue}};

#[derive(Debug, Clone)]
pub struct ScopeDetector {
    pub start: Box<Detector>,
    pub end: Box<Detector>,
}

impl ScopeDetector {
    pub fn new(start: Box<Detector>, end: Box<Detector>) -> Self {
        Self {
            start,
            end,
        }
    }
}

impl Detectable for ScopeDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        let mut inner: Queue = vec![];
        let start_result;

        match queue.consume(self.start.as_ref()) {
            (true, _, Some(result)) => {
                start_result = result;
            },
            _ => return None
        }

        let mut layer = 1;

        while !queue.is_empty() {
            let mut end_queue = queue.clone();
            let mut start_queue = queue.clone();

            let end_queue_result = end_queue.consume(self.end.as_ref());
            let start_queue_result = start_queue.consume(self.start.as_ref());

            match (end_queue_result, start_queue_result) {
                ((true, _, Some(end_result)), _) => {
                    layer -= 1;

                    let consumed = queue.len() - end_queue.len();

                    if layer == 0 {
                        for _ in 0..consumed {
                            queue.remove(0);
                        }

                        let mut result = Result::new(Detector::ScopeDetector(self.clone()), Some(inner), None, None);

                        result.properties = Some(
                            Dict::from(
                                vec![
                                    ("start".to_string(), &start_result),
                                    ("end".to_string(), &end_result)
                                ]
                            )
                        );

                        return Some(result);
                    }

                    for _ in 0..consumed {
                        inner.push(
                            queue.remove(0)
                        );
                    }
                },
                ((false, _, _), (true, _, Some(_))) => {
                    layer += 1;

                    let consumed = queue.len() - start_queue.len();

                    for _ in 0..consumed {
                        inner.push(
                            queue.remove(0)
                        );
                    }
                },
                _ => {
                    if !queue.is_empty() {
                        inner.push(queue.remove(0));
                    }
                }
            }
        }

        None
    }
}

impl PartialEq for ScopeDetector {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[cfg(test)]
mod tests {
    use crate::{detectors::word_detector::WordDetector, types::Queue};

    use super::*;

    #[test]
    fn test_scope_detector() {
        let mut queue = Queue::from(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);

        let start = Detector::WordDetector(WordDetector::new(Some("a".to_string()), None, None));
        let end = Detector::WordDetector(WordDetector::new(Some("g".to_string()), None, None));

        let detector = Detector::ScopeDetector(ScopeDetector::new(Box::new(start), Box::new(end)));

        let (matched, _, result) = queue.consume(&detector);

        assert_eq!(matched, true);

        match result {
            Some(result) => {
                assert_eq!(result.content, Some(vec!['b', 'c', 'd', 'e', 'f']));
            },
            _ => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_scope_detector_nested() {
        let mut queue = Queue::from(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);

        let start = Detector::WordDetector(WordDetector::new(Some("a".to_string()), None, None));
        let end = Detector::WordDetector(WordDetector::new(Some("h".to_string()), None, None));

        let detector = Detector::ScopeDetector(ScopeDetector::new(Box::new(start), Box::new(end)));

        let (matched, _, result) = queue.consume(&detector);

        assert_eq!(matched, true);

        match result {
            Some(result) => {
                assert_eq!(result.content, Some(vec!['b', 'c', 'd', 'e', 'f', 'g']));
            },
            _ => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_scope_detector_nested_multiple() {
        let mut queue = Queue::from(vec!['a', 'b', 'c', 'd', 'e', 'f', 'a', 'v', 'h', 'b', 'c', 'a', 'h', 'f', 'g', 'h']);

        let start = Detector::WordDetector(WordDetector::new(Some("a".to_string()), None, None));
        let end = Detector::WordDetector(WordDetector::new(Some("h".to_string()), None, None));

        let detector = Detector::ScopeDetector(ScopeDetector::new(Box::new(start), Box::new(end)));

        let (matched, _, result) = queue.consume(&detector);

        assert_eq!(matched, true);

        match result {
            Some(result) => {
                assert_eq!(result.content, Some(vec!['b', 'c', 'd', 'e', 'f', 'a', 'v', 'h', 'b', 'c', 'a', 'h', 'f', 'g']));
            },
            _ => {
                assert_eq!(true, false);
            }
        }
    }
}