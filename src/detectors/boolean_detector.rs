// Boolean Detector: Have Any/All/None of the sub-detectors matching
// ----------------------------------------------------------

use crate::{base::*, types::Queue};

// Enum
#[derive(Debug, Clone)]
pub enum BooleanDetector {
    Any(Vec<Detector>),
    All(Vec<Detector>),
    None(Vec<Detector>),
    Not(Box<Detector>)
}

impl Detectable for BooleanDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        match self {
            BooleanDetector::Any(detectors) => {
                for detector in detectors {
                    let mut queue_clone = queue.clone();

                    match queue_clone.consume(detector) {
                        (true, _, Some(result)) => {
                            return Some(result);
                        },
                        _ => {}
                    }
                }

                None
            },
            BooleanDetector::All(detectors) => {
                let mut children = vec![];

                for detector in detectors {
                    match queue.clone().consume(detector) {
                        (true, _, Some(result)) => {
                            children.push(result);
                        },
                        _ => return None
                    }
                }

                let result = Result::new(Detector::BooleanDetector(self.clone()), Some(queue.clone()), None, Some(children));

                Some(result)
            },
            BooleanDetector::None(detectors) => {
                for detector in detectors {
                    let mut queue_clone = queue.clone();

                    match queue_clone.consume(detector) {
                        (true, _, Some(_)) => {
                            return None;
                        },
                        _ => {}
                    }
                }

                let result = Result::new(Detector::BooleanDetector(self.clone()), Some(queue.clone()), None, None);

                Some(result)
            },
            BooleanDetector::Not(detector) => {
                let mut queue_clone = queue.clone();

                match queue_clone.consume(detector) {
                    (true, _, Some(_)) => {
                        return None;
                    },
                    _ => {}
                }

                let result = Result::new(Detector::BooleanDetector(self.clone()), Some(queue.clone()), None, None);

                Some(result)
            }
        }
    }
}

impl PartialEq for BooleanDetector {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BooleanDetector::Any(detectors), BooleanDetector::Any(other_detectors)) => {
                detectors == other_detectors
            },
            (BooleanDetector::All(detectors), BooleanDetector::All(other_detectors)) => {
                detectors == other_detectors
            },
            (BooleanDetector::None(detectors), BooleanDetector::None(other_detectors)) => {
                detectors == other_detectors
            },
            (BooleanDetector::Not(detector), BooleanDetector::Not(other_detector)) => {
                detector == other_detector
            },
            _ => false
        }
    }
}