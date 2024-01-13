// Div Parser
// -----------------------------------------------------------------------------

use crate::advanced_detectors::tag_scope_detector::*;

pub fn div_detector() -> TagScopeDetector {
    TagScopeDetector::new(
        Some("div".to_string()),
        None,
        None,
        None,
        Some(false),
        None
    )
}