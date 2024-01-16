// =============================================
// Tag Detector
// =============================================

// External imports
/// Regex
use fancy_regex::{Regex, Captures};

// Crate imports
use crate::types::*;
use crate::tag::Tag;

use super::attributes_detector::detect_attributes;

// Tag Detector (Detects a single tag (opening or closing))
#[derive(Debug, Clone)]
pub struct TagDetector {
    pub name: Option<String>,
    pub attributes: Option<Dict>,
    pub is_closing: bool,
    pub allow_self_closing: Option<bool>,
}

impl TagDetector {
    pub fn new(name: Option<String>, attributes: Option<Dict>, is_closing: bool, allow_self_closing: Option<bool>) -> Self {
        Self {
            name,
            attributes,
            is_closing,
            allow_self_closing,
        }
    }

    pub fn new_opening(name: Option<String>, attributes: Option<Dict>, allow_self_closing: Option<bool>) -> Self {
        Self::new(name, attributes, false, allow_self_closing)
    }

    pub fn new_closing(name: Option<String>) -> Self {
        Self::new(name, None, true, None)
    }
}

pub fn extract_data(cap: &Captures) -> (String, Dict) {
    // Get name
    let name = match cap.name("name") {
        Some(name) => name.as_str().to_string(),
        None => String::new(),
    };

    // Get attributes
    let attributes_string = match cap.name("attributes") {
        Some(attributes) => attributes.as_str().to_string(),
        None => String::new(),
    };

    let attributes = detect_attributes(&mut Queue::from(attributes_string));

    (name, attributes)
}

impl Detectable for TagDetector {
    fn regex(&self, _: &mut Queue) -> String {
        // Craft Regex for attributes
        let mut attributes: String = match &self.attributes {
            Some(attributes) => {
                let mut attributes_string = String::new();

                for (key, value) in attributes.clone().iter() {
                    attributes_string.push_str(&format!(r#"(?P<{}>\s*{}(?:\s*=\s*(?P<{}>"(?:[^"]|\\")*"))?)?"#, key, key, value));
                }

                attributes_string
            },
            None => String::from("[^>]"),
        };
        
        attributes = format!(
            r#"(?P<attributes>\s{}*?)?"#,
            attributes
        );

        let name = match &self.name {
            Some(name) => format!(r#"{}"#, name),
            None => String::from(r#"(?P<name>[a-zA-Z]+[a-zA-Z0-9]*)"#),
        };

        // Check if the tag is closing
        let is_closing = self.is_closing;

        // Check if the tag is self closing
        let allow_self_closing = self.allow_self_closing;

        // Craft Regex
        let regex_opening = format!(
            r#"(?<tag><\s?{}{}\s?>)"#,
            name,
            attributes
        );

        let regex_closing = format!(
            r#"(?<tag><\/\s?{}\s?>)"#,
            name
        );

        let regex_self_closing = format!(
            r#"(?<tag><\s?{}\s{}\s?/>)"#,
            name,
            attributes
        );

        // Return the correct Regex
        if is_closing {
            regex_closing
        } else if allow_self_closing.unwrap_or(false) {
            regex_self_closing
        } else {
            regex_opening
        }
    }

    fn detect(&self, queue: &mut Queue) -> Option<(String, Tag)> {
        // Craft Regex for attributes
        let regex = self.regex(&mut queue.clone());

        // Enclose to match at start of string
        let regex = format!("^{}", regex);

        // Compile Regex
        let regex = Regex::new(&regex).unwrap();

        // Match Regex and get groups
        let groups = match regex.captures(queue.as_str()) {
            Ok(groups) => groups,
            Err(_) => return None
        }?;

        let (name, attributes) = extract_data(&groups);

        let tag = Tag::new(name, Some(attributes), &None::<Vec<Tag>>);

        queue.drain(0..groups.name("tag").unwrap().end());

        Some((String::new(), tag))
    }
}