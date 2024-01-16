// Detect Text Styling like bold, italic, underline, etc. (*, **, _, ~, etc.)
// and convert them to HTML tags.

use crate::base::{Detector, Detectable, Result};
use crate::types::Queue;
use crate::advanced_detectors::scope_detector::ScopeDetector;

pub struct TextStyleDetector;

fn detect_bold(queue: &Queue) -> Option<Result> {
    let mut queue = queue.clone();

    let scope_detector = ScopeDetector::new(
        Detector::WordDetector("**".to_string()),
        Detector::WordDetector("**".to_string())
    );

    let (matched, _, result) = queue.consume(&scope_detector);

    // TODO
}

impl Detectable for TextStyleDetector {
    fn detect(&self, queue: &Queue) -> Option<Result> {
        
    }
}