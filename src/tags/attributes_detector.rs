// =================================
// Detect Attributes (from a tag)
// =================================

use regex::Regex;

use crate::types::*;


// Attributes detector
pub fn detect_attributes(queue: &mut Queue) -> Dict {
    // Regex for a single attribute
    let attribute_regex = Regex::new(r#"(?P<key>[a-zA-Z0-9]+)(?:\s*=\s*"(?P<value>(?:[^"]|\\")*)")?"#).unwrap();

    let mut dict = Dict::new();

    loop {
        let clone = queue.clone();

        let matches = match attribute_regex.captures(clone.as_str()) {
            Some(matches) => matches,
            None => break,
        };

        // Get key and value
        let key = matches.name("key").unwrap().as_str();
        let value = String::from(matches.name("value").unwrap().as_str());

        // Remove the attribute from the consumable
        queue.drain(matches.get(0).unwrap().range());

        // Add the attribute to the dict
        dict.set(key, &value);
    }

    // Return attributes
    dict
}