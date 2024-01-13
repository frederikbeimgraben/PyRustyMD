// Detect a html tag scope
// -----------------------

use regex::Regex;

use crate::base::*;
use crate::advanced_detectors::tag_detector::TagDetector;
use crate::detectors::scope_detector::ScopeDetector;
use crate::detectors::word_detector::whitespace_detector;
use crate::types::{Queue, Value, Dict};

#[derive(Debug, Clone)]
pub struct TagScopeDetector {
    pub tag: Option<Regex>,
    pub id: Option<String>,
    pub class: Option<Vec<String>>,
    pub allow_inner: Option<bool>,
    pub is_standalone: Option<bool>, // Like <img>
    pub allow_self_closing: Option<bool>
}

impl TagScopeDetector {
    pub fn new(tag: Option<String>, id: Option<String>, class: Option<Vec<String>>, allow_inner: Option<bool>, is_standalone: Option<bool>, allow_self_closing: Option<bool>) -> Self {
        Self {
            tag: match tag {
                Some(tag) => Some(Regex::new(&format!(r"^{}$", tag)).unwrap()),
                None => None
            },
            id,
            class,
            allow_inner,
            is_standalone,
            allow_self_closing
        }
    }

    pub fn new_regex(tag: Option<Regex>, id: Option<String>, class: Option<Vec<String>>, allow_inner: Option<bool>, is_standalone: Option<bool>, allow_self_closing: Option<bool>) -> Self {
        Self {
            tag,
            id,
            class,
            allow_inner,
            is_standalone,
            allow_self_closing
        }
    }
}

impl Detectable for TagScopeDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        let whitespace_detector = Detector::WordDetector(whitespace_detector());

        // Consume Whitespace
        queue.consume(&whitespace_detector);

        // Start Tag: either opening or self-closing
        let start_tag_detector = TagDetector::new_regex(
            self.tag.clone(),
            None,
            Some(false),
            None,
            None
        );

        let (start_tag_detected, _, start_tag_result) = queue.clone().consume(&Detector::TagDetector(start_tag_detector));

        if !start_tag_detected {
            return None;
        }

        let attributes;
        let is_self_closing;
        let tag_name;

        match start_tag_result {
            Some(result) => {
                (attributes, is_self_closing, tag_name) = match (
                    result.get_property("attributes"),
                    result.get_property("self_closing"),
                    result.get_property("tag")
                ) {
                    (
                        Value::Dict(attributes),
                        Value::Boolean(is_self_closing),
                        Value::String(tag_name)
                    ) => {
                        (attributes, is_self_closing, tag_name)
                    },
                    _ => return None
                };
            },
            None => return None
        }

        if !self.allow_self_closing.unwrap_or(true) && is_self_closing {
            return None;
        }

        let class = match attributes.get("class") {
            Value::String(class) => class.clone(),
            _ => "".to_string()
        };

        let classes_string = class.clone();

        let classes: Vec<Value> = classes_string.split(" ").map(|class| Value::String(class.to_string())).collect();

        let id = match attributes.get("id") {
            Value::String(id) => id.clone(),
            _ => "".to_string()
        };

        let style = match attributes.get("style") {
            Value::String(style) => style.clone(),
            _ => "".to_string()
        };

        if self.id.is_some() && self.id != Some(id.clone()) {
            return None;
        }

        if self.class.is_some() {
            for class in self.class.clone().unwrap() {
                if !class.contains(&class) {
                    return None;
                }
            }
        }

        // If self-closing, return result
        if is_self_closing || self.is_standalone.unwrap_or(false) {
            // Consume start tag
            let start_tag_detector = TagDetector::new(
                Some(tag_name.clone()),
                None,
                Some(false),
                Some(true),
                None
            );

            queue.consume(&Detector::TagDetector(start_tag_detector));

            let properties = Dict::from(
                vec![
                    ("tag".to_string(), &tag_name),
                    ("id".to_string(), &id),
                    ("classes".to_string(), &classes),
                    ("style".to_string(), &style),
                    ("is_self_closing".to_string(), &is_self_closing),
                    ("attributes".to_string(), &attributes)
                ]
            );

            let result = Result::new(Detector::TagScopeDetector(self.clone()), None, Some(properties), None);

            return Some(result);
        }

        // Inner start detector (for counting inner tags)
        let inner_start_tag_detector = TagDetector::new(
            Some(tag_name.clone()),
            None,
            Some(false),
            None,
            None
        );

        // Inner end detector (for counting inner tags)
        let inner_end_tag_detector = TagDetector::new(
            Some(tag_name.clone()),
            None,
            Some(true),
            None,
            None
        );

        // Inner scope detector
        let inner_scope_detector = ScopeDetector::new(
            Box::new(Detector::TagDetector(inner_start_tag_detector)),
            Box::new(Detector::TagDetector(inner_end_tag_detector)),
        );

        // Get Scope Result
        let (inner_scope_detected, _, inner_scope_result) = queue.consume(&Detector::ScopeDetector(inner_scope_detector));

        if !inner_scope_detected {
            return None;
        }

        // Set inner as inner scope result content
        let inner = match inner_scope_result {
            Some(result) => match result.content {
                Some(inner) => inner,
                None => return None
            },
            None => return None
        };

        if !self.allow_inner.unwrap_or(false) && inner.len() > 0 {
            return None;
        }

        // Fill Result
        let properties = Dict::from(
            vec![
                ("tag".to_string(), &tag_name),
                ("id".to_string(), &id),
                ("classes".to_string(), &classes),
                ("style".to_string(), &style),
                ("is_self_closing".to_string(), &is_self_closing),
                ("attributes".to_string(), &attributes)
            ]
        );

        let result = Result::new(Detector::TagScopeDetector(self.clone()), Some(inner), Some(properties), None);

        Some(result)
    }
}

impl PartialEq for TagScopeDetector {
    fn eq(&self, other: &Self) -> bool {
        (
            self.tag.clone().unwrap_or(Regex::new(r"").ok().unwrap()).as_str() == 
            other.tag.clone().unwrap_or(Regex::new(r"").ok().unwrap()).as_str()
        ) &&
        self.id == other.id &&
        self.class == other.class
    }
}