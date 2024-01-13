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
        Some(true),
        Some(
            vec![ // src, alt, width, height, style, class, id
                ("src".to_string(), None),
                ("alt".to_string(), None),
                ("width".to_string(), None),
                ("height".to_string(), None),
                ("style".to_string(), None),
                ("class".to_string(), None),
                ("id".to_string(), None)
            ]
        )
    )
}