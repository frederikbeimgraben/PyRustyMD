// H Tag Detector
// ------------------------

use regex::Regex;
use crate::advanced_detectors::tag_scope_detector::*;

pub fn h_detector() -> TagScopeDetector {
    TagScopeDetector::new_regex(
        Regex::new(r"^h[1-6]$").ok(),
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