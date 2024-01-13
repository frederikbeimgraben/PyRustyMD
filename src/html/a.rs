// A Tag Detector
// ------------------------

use crate::advanced_detectors::tag_scope_detector::*;

pub fn a_detector() -> TagScopeDetector {
    TagScopeDetector::new(
        Some("a".to_string()),
        None,
        None,
        None,
        Some(false),
        None
    )
}