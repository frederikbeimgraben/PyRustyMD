// =================================
// This file contains the TagScopeDetector struct, which is used to detect the
// scope of a tag (i.e. the start and end of a tag).
// =================================

use regex::Regex;

use crate::types::*;

use super::tag_detector::{TagDetector, extract_data};

// Tag Scope Detector
#[derive(Debug, Clone)]
pub struct TagScopeDetector {
    pub name: Option<String>,
    pub attributes: Option<Dict>,
    pub allow_self_closing: Option<bool>,
    pub allow_content: Option<bool>,
    pub is_standalone: Option<bool>, // eg <br>, <hr>, <img>
}

impl TagScopeDetector {
    pub fn new(name: Option<String>, attributes: Option<Dict>, allow_self_closing: Option<bool>, allow_content: Option<bool>, is_standalone: Option<bool>) -> Self {
        Self {
            name,
            attributes,
            allow_self_closing,
            allow_content,
            is_standalone
        }
    }

    pub fn new_standalone(name: Option<String>, attributes: Option<Dict>) -> Self {
        Self::new(name, attributes, None, Some(false), Some(true))
    }

    pub fn new_opening(name: Option<String>, attributes: Option<Dict>, allow_self_closing: Option<bool>, allow_content: Option<bool>) -> Self {
        Self::new(name, attributes, allow_self_closing, allow_content, None)
    }

    pub fn new_closing(name: Option<String>) -> Self {
        Self::new(name, None, None, None, None)
    }
}

impl Detectable for TagScopeDetector {
    fn regex(&self, queue: &mut Queue) -> String {
        // Craft Regex for attributes
        let opening_tag = TagDetector::new_opening(self.name.clone(), self.attributes.clone(), self.allow_self_closing.clone());
        
        // Match the opening tag, to get the name
        let name = match opening_tag.detect(queue) {
            Some((_, tag)) => tag.name,
            None => r"^\b$".to_string()
        };
        
        let closing_tag = TagDetector::new_closing(Some(name));

        let opening_regex = opening_tag.regex(&mut queue.clone());
        let closing_regex = closing_tag.regex(&mut queue.clone());

        // Craft Regex for content
        let content = match &self.allow_content {
            Some(allow_content) => {
                if *allow_content {
                    r#"(?P<content>[\s\S]*?)?"#
                } else {
                    r#"\s*?"#
                }
            },
            None => r#"(?P<content>[\s\S]*?)?"#,
        };

        // Craft Regex for standalone tags
        let standalone = format!(
            r#"(?P<standalone>{})"#,
            opening_regex,
        );

        // Craft Regex
        let regex = format!(
            r#"(?P<opening>{}){}(?P<closing>{})"#,
            opening_regex,
            content,
            closing_regex,
        );

        // Return Regex
        match &self.is_standalone {
            Some(is_standalone) => {
                if *is_standalone {
                    standalone
                } else {
                    regex
                }
            },
            None => regex,
        }
    }

    fn detect(&self, queue: &mut Queue) -> Option<(String, crate::tag::Tag)> {
        // Get regex
        let regex = self.regex(&mut queue.clone());

        // Get matches
        let matches = match Regex::new(&regex) {
            Ok(regex) => {
                regex.captures(queue.as_str())
            },
            Err(_) => {
                None
            },
        };

        // Check if there are matches
        let matches = match matches {
            Some(matches) => matches,
            None => return None,
        };

        // Get name and attributes
        let (name, attributes) = extract_data(&matches);

        // Get content
        let content = matches.name("content").map(|content| content.as_str().to_string());

        // Create tag
        let mut tag = crate::tag::Tag::init(
            name, Some(attributes)
        );

        // Drain the tag from the queue
        queue.drain(matches.get(0).unwrap().range());

        // Return the tag
        Some((content.unwrap_or(String::new()), tag))
    }
}