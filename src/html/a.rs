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
        None,
        Some(
            vec![ // hreflang, download, target, title, href, name, style, class, id
                ("hreflang".to_string(), None),
                ("download".to_string(), None),
                ("target".to_string(), None),
                ("title".to_string(), None),
                ("href".to_string(), None),
                ("name".to_string(), None),
                ("style".to_string(), None),
                ("class".to_string(), None),
                ("id".to_string(), None)
            ]
        )
    )
}