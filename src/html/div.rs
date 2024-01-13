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
        None,
        Some(
            vec![ // style, class, id
                ("style".to_string(), None),
                ("class".to_string(), None),
                ("id".to_string(), None)
            ]
        )
    )
}