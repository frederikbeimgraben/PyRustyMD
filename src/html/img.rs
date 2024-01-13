// IMG Tag Detector
// ------------------------

use crate::advanced_detectors::tag_scope_detector::*;

pub fn img_detector() -> TagScopeDetector {
    TagScopeDetector::new(
        Some("img".to_string()),
        None,
        None,
        None,
        Some(true),
        Some(true)
    )
}