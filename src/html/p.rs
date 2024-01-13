// P Tag Detector
// ------------------------

use crate::advanced_detectors::tag_scope_detector::*;

pub fn p_detector() -> TagScopeDetector {
    TagScopeDetector::new(
        Some("p".to_string()),
        None,
        None,
        None,
        Some(false),
        None
    )
}