// Span Tag
// ---------------

use crate::advanced_detectors::tag_scope_detector::*;

pub fn span_detector() -> TagScopeDetector {
    TagScopeDetector::new(
        Some("span".to_string()),
        None,
        None,
        Some(true),
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