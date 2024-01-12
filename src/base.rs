// Base Module: Supertraits, Types etc.
// -----------------------------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------------------------

use std::{fmt::Debug, vec};

use crate::{
    detectors::{
        number_detector::NumberDetector,
        word_detector::WordDetector,
        scope_detector::ScopeDetector,
        property_detector::PropertyDetector, boolean_detector::BooleanDetector
    }, 
    advanced_detectors::{tag_detector::TagDetector, tag_scope_detector::TagScopeDetector},
    types::*
};

/// Result Trait (Result of a detection)
#[derive(Debug, Clone)]
pub struct Result {
    pub detector: Detector,

    // If matched is true, then the following fields can be used
    pub content: Option<Queue>,
    pub properties: Option<Dict>,

    pub children: Option<Vec<Result>>,
}

impl Result {
    pub fn new(detector: Detector, content: Option<Queue>, properties: Option<Dict>, children: Option<Vec<Result>>) -> Self {
        Self {
            detector,
            content,
            properties,
            children
        }
    }

    pub fn get_property(&self, key: &str) -> Value {
        match &self.properties {
            Some(properties) => properties.get(key),
            None => Value::NoneValue
        }
    }
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        self.detector == other.detector &&
        self.content == other.content &&
        self.properties == other.properties &&
        self.children == other.children
    }
}

/// Detectable Trait (A object that can detect a pattern from a queue)
pub trait Detectable: Debug + Clone + PartialEq {
    fn detect(&self, queue: &mut Queue) -> Option<Result>;
}

pub trait Consumable {
    fn consume(&mut self, detector: &Detector) -> (bool, Option<String>, Option<Result>);
    fn consume_any(&mut self, detectors: &Vec<Detector>) -> Option<Vec<Result>> ;
    fn from_string(string: String) -> Self;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Detector {
    NumberDetector(NumberDetector),
    WordDetector(WordDetector),
    ScopeDetector(ScopeDetector),
    PropertyDetector(PropertyDetector),
    BooleanDetector(BooleanDetector),
    TagDetector(TagDetector),
    TagScopeDetector(TagScopeDetector),
    RawDetector,
    NoneDetector
}

impl PartialEq for Detector {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NumberDetector(number_detector_1), Self::NumberDetector(number_detector_2)) => number_detector_1 == number_detector_2,
            (Self::WordDetector(word_detector_1), Self::WordDetector(word_detector_2)) => word_detector_1 == word_detector_2,
            (Self::ScopeDetector(scope_detector_1), Self::ScopeDetector(scope_detector_2)) => scope_detector_1 == scope_detector_2,
            (Self::PropertyDetector(property_detector_1), Self::PropertyDetector(property_detector_2)) => property_detector_1 == property_detector_2,
            (Self::BooleanDetector(boolean_detector_1), Self::BooleanDetector(boolean_detector_2)) => boolean_detector_1 == boolean_detector_2,
            (Self::TagDetector(tag_detector_1), Self::TagDetector(tag_detector_2)) => tag_detector_1 == tag_detector_2,
            (Self::TagScopeDetector(tag_scope_detector_1), Self::TagScopeDetector(tag_scope_detector_2)) => tag_scope_detector_1 == tag_scope_detector_2,
            (Self::NoneDetector, Self::NoneDetector) => true,
            _ => false
        }
    }
}

impl Detectable for Detector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        match self {
            Self::NumberDetector(number_detector) => number_detector.detect(queue),
            Self::WordDetector(word_detector) => word_detector.detect(queue),
            Self::ScopeDetector(scope_detector) => scope_detector.detect(queue),
            Self::PropertyDetector(property_detector) => property_detector.detect(queue),
            Self::BooleanDetector(boolean_detector) => boolean_detector.detect(queue),
            Self::TagDetector(tag_detector) => tag_detector.detect(queue),
            Self::TagScopeDetector(tag_scope_detector) => tag_scope_detector.detect(queue),
            Self::RawDetector => None,
            Self::NoneDetector => None
        }
    }
}

impl Consumable for Queue {
    fn consume(&mut self, detector: &Detector) -> (bool, Option<String>, Option<Result>) {
        let mut copy = self.clone();

        match detector.detect(&mut copy) {
            Some(result) => {
                // Consume from the queue
                let consumed = self.len() - copy.len();

                let buffer = self[0..consumed].iter().collect::<String>();

                for _ in 0..consumed {
                    self.remove(0);
                }

                (
                    true,
                    Some(
                        buffer
                    ), 
                    Some(result)
                )
            },
            None => (false, None, None)
        }
    }

    // Consume the whole queue, also consuming the content of a rsult and setting it to children
    fn consume_any(&mut self, detectors: &Vec<Detector>) -> Option<Vec<Result>> {
        let mut buffer = vec![];

        let mut children = vec![];

        while self.len() > 0 {
            let mut found: bool = false;

            for detector in detectors {
                let mut copy = self.clone();

                match detector.detect(&mut copy) {
                    Some(mut result) => {
                        // Handle Raw Buffer
                        found = true;

                        if buffer.len() > 0 {
                            children.push(
                                Result::new(
                                    Detector::RawDetector,
                                    Some(buffer.clone()),
                                    None,
                                    None
                                )
                            );

                            buffer = vec![];
                        }

                        // Consume from the queue
                        let consumed = self.len() - copy.len();

                        for _ in 0..consumed {
                            self.remove(0);
                        }

                        // Get result content
                        match result.clone().content {
                            Some(content) => {
                                // If content is not empty, consume it recursively
                                let mut content_queue = content.clone();

                                if content_queue.len() > 0 {
                                    let subchildren = content_queue.consume_any(detectors);

                                    result.children = match subchildren {
                                        Some(subchildren) => Some(subchildren),
                                        None => None
                                    };
                                }
                            },
                            None => {}
                        }

                        children.push(result.clone());
                    },
                    None => {}
                }
            }

            if !found {
                buffer.push(self.remove(0));
            }
        }

        if buffer.len() > 0 {
            children.push(
                Result::new(
                    Detector::RawDetector,
                    Some(buffer.clone()),
                    None,
                    None
                )
            );
        }

        if children.len() > 0 {
            Some(children)
        } else {
            None
        }
    }

    fn from_string(string: String) -> Self {
        string.chars().collect()
    }

    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}