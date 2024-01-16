// Types needed for the detection
// -----------------------------------------------------------------------------------------------

use std::{collections::HashMap, any::Any, fmt::Debug};

use pyo3::{IntoPy, Python, PyObject, types::PyDict};

use crate::tag::Tag;

// ===============================================================================================
// Types
// ===============================================================================================

pub trait QueueTrait: Any + Debug + Clone {
    fn consume(&mut self, detector: &Detector) -> Option<Result>;
    fn next(&mut self) -> Option<char>;
    fn consume_all(&mut self, detectors: Vec<&Detector>) -> Result;
}

pub type Queue = String;

impl QueueTrait for Queue {
    fn consume(&mut self, detector: &Detector) -> Option<Result> {
        let mut queue = self.clone();

        let result = detector.detect(&mut queue);

        let consumed: usize = queue.len() - self.len();

        match result {
            Some(result) => {
                self.drain(0..consumed);

                Some(result)
            },
            None => None
        }
    }

    fn next(&mut self) -> Option<char> {
        self.chars().next()
    }

    fn consume_all(&mut self, detectors: Vec<&Detector>) -> Result {
        let mut results: Vec<Result> = vec![];

        let mut buffer: String = String::new();

        loop {
            let det_results = detectors.iter().map(|detector| self.clone().consume(detector)).collect::<Vec<Option<Result>>>();

            // Get the first result that is not None
            let result = det_results.iter().find(|result| result.is_some());

            match result {
                Some(result) => {
                    // Unpack buffer and push it to the results
                    if buffer.len() > 0 {
                        let mut buffer_result = Result::new(
                            None,
                            None,
                            None,
                            None,
                            Some(buffer),
                            None
                        );

                        buffer_result.flag(Flag::Raw);

                        results.push(buffer_result);

                        buffer = String::new();
                    }

                    // Push the result to the results
                    if let Some(result) = result.clone() {
                        results.push(result);
                    }
                },
                None => {
                    // Push the first character of the queue to the buffer
                    match self.next() {
                        Some(character) => {
                            buffer.push(character);
                        },
                        None => {
                            // Unpack buffer and push it to the results
                            if buffer.len() > 0 {
                                let mut buffer_result = Result::new(
                                    None,
                                    None,
                                    None,
                                    None,
                                    Some(buffer),
                                    None
                                );

                                buffer_result.flag(Flag::Raw);

                                results.push(buffer_result);
                            }

                            break;
                        }
                    }
                }
            }
        }

        let mut result = Result::new(
            None,
            None,
            None,
            None,
            None,
            Some(results)
        );

        result.flag(Flag::Root);

        result
    }
}

// -----------------------------------------------------------------------------------------------
// Result (of a detection)
// -----------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Flag {
    Raw,
    Root
}

#[derive(Debug, Clone)]
pub struct Result {
    pub tag: Option<String>,
    pub attributes: Dict,
    pub options: Dict,
    pub flags: HashMap<Flag, bool>,
    pub content: Option<Queue>,
    pub raw: Option<String>,
    pub children: Option<Vec<Result>>
}

impl Result {
    pub fn new(tag: Option<String>, attributes: Option<Dict>, options: Option<Dict>, content: Option<Queue>, raw: Option<String>, children: Option<Vec<Result>>) -> Self {
        Self {
            tag,
            attributes: attributes.unwrap_or(Dict::new()),
            options: options.unwrap_or(Dict::new()),
            flags: HashMap::new(),
            content,
            raw,
            children
        }
    }

    // Accessors
    /// Attributes
    pub fn get_attribute(&self, key: &str) -> Value {
        self.attributes.get(key)
    }

    pub fn set_attribute(&mut self, key: &str, value: &(dyn Any)) {
        self.attributes.set(key, value);
    }

    pub fn get_attributes(&self) -> Dict {
        self.attributes.clone()
    }

    pub fn set_attributes(&mut self, attributes: Dict) {
        self.attributes = attributes;
    }

    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.has(key)
    }

    /// Options
    pub fn get_option(&self, key: &str) -> Value {
        self.options.get(key)
    }

    pub fn set_option(&mut self, key: &str, value: &(dyn Any)) {
        self.options.set(key, value);
    }

    pub fn has_option(&self, key: &str) -> bool {
        self.options.has(key)
    }

    /// Flags
    pub fn flag(&mut self, key: Flag) {
        self.flags.insert(key, true);
    }

    pub fn unflag(&mut self, key: Flag) {
        self.flags.insert(key, false);
    }

    pub fn is_flagged(&self, key: Flag) -> bool {
        match self.flags.get(&key) {
            Some(flag) => *flag,
            None => false
        }
    }
}

impl IntoPy<PyObject> for Result {
    fn into_py(self, py: Python) -> PyObject {
        let mut properties = Dict::new();

        let children: Vec<Value> = match &self.children {
            Some(children) => children.iter().map(|child| Value::Result(child.clone())).collect(),
            None => vec![]
        };

        properties.set(
            "tag",
            &self.tag
        );

        properties.set(
            "attributes",
            &self.get_attributes()
        );

        properties.set(
            "options",
            &self.options
        );

        match self.is_flagged(Flag::Raw) {
            true => {
                self.content.unwrap_or(Queue::new()).to_string().into_py(py)
            },
            _ => {
                properties.set(
                    "content",
                    &Value::Array(children)
                );

                properties.into_py(py)
            }
        }
    }
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag &&
        self.attributes == other.attributes &&
        self.options == other.options &&
        self.flags == other.flags &&
        self.content == other.content &&
        self.raw == other.raw &&
        self.children == other.children
    }
}

// -----------------------------------------------------------------------------------------------
// Detectable
// -----------------------------------------------------------------------------------------------

pub trait Detectable {
    fn detect(&self, queue: &mut Queue) -> Option<Result>;
}

pub enum Detector<'a> {
    Any(&'a(dyn Detectable)),
    None
}

impl Detectable for Detector<'_> {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        match self {
            Self::Any(detector) => detector.detect(queue),
            Self::None => None
        }
    }
}

// -----------------------------------------------------------------------------------------------
// Value
// -----------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Value {
    // None
    NoneValue,

    // Primitive Types
    String(String),
    Boolean(bool),
    Float(f64),
    Integer(i64),

    // Application Types
    Queue(Queue),
    Result(Result),
    Tag(Tag),

    // Collections
    Dict(Dict),
    Array(Vec<Value>)
}

impl Value {
    pub fn new(ref value: &(dyn Any)) -> Self {
        match value {
            value if value.is::<String>()     => Self::String(value.downcast_ref::<String>().unwrap().clone()),
            value if value.is::<bool>()       => Self::Boolean(value.downcast_ref::<bool>().unwrap().clone()),
            value if value.is::<f64>()        => Self::Float(value.downcast_ref::<f64>().unwrap().clone()),
            value if value.is::<i64>()        => Self::Integer(value.downcast_ref::<i64>().unwrap().clone()),
            value if value.is::<Queue>()      => Self::Queue(value.downcast_ref::<Queue>().unwrap().clone()),
            value if value.is::<Dict>()       => Self::Dict(value.downcast_ref::<Dict>().unwrap().clone()),
            value if value.is::<Vec<Value>>() => Self::Array(value.downcast_ref::<Vec<Value>>().unwrap().clone()),
            value if value.is::<Result>()     => Self::Result(value.downcast_ref::<Result>().unwrap().clone()),
            value if value.is::<Tag>()        => Self::Tag(value.downcast_ref::<Tag>().unwrap().clone()),

            // Array of any type
            value if value.is::<Vec<&(dyn Any)>>() => {
                let mut array: Vec<Value> = vec![];

                for value in value.downcast_ref::<Vec<&(dyn Any)>>().unwrap() {
                    array.push(Self::new(value));
                }

                Self::Array(array)
            },

            // Value
            value if value.is::<Value>() => value.downcast_ref::<Value>().unwrap().clone(),

            // Any other type
            _ => Self::NoneValue,
        }
    }

    pub fn from(value: Value) -> Self {
        match value {
            // Primitive Types
            Self::String(string) => Self::String(string),
            Self::Boolean(boolean) => Self::Boolean(boolean),
            Self::Float(number) => Self::Float(number),
            Self::Integer(number) => Self::Integer(number),

            // Application Types
            Self::Queue(queue) => Self::Queue(queue),
            Self::Result(result) => Self::Result(result),
            Self::Tag(tag) => Self::Tag(tag),

            // Collections
            Self::Dict(properties) => Self::Dict(properties),
            Self::Array(array) => Self::Array(array),

            // None
            Self::NoneValue => Self::NoneValue
        }
    }

    pub fn value(&self) -> Option<&dyn Any> {
        match self {
            // Primitive Types
            Self::String(string) => Some(string),
            Self::Boolean(boolean) => Some(boolean),
            Self::Float(number) => Some(number),
            Self::Integer(number) => Some(number),

            // Application Types
            Self::Result(result) => Some(result),
            Self::Queue(queue) => Some(queue),
            Self::Tag(tag) => Some(tag),

            // Collections
            Self::Dict(properties) => Some(properties),
            Self::Array(array) => Some(array),

            // None
            Self::NoneValue => None
        }
    }
} 

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(string_1), Self::String(string_2)) => string_1 == string_2,
            (Self::Boolean(boolean_1), Self::Boolean(boolean_2)) => boolean_1 == boolean_2,
            (Self::Float(number_1), Self::Float(number_2)) => number_1 == number_2,
            (Self::Integer(number_1), Self::Integer(number_2)) => number_1 == number_2,
            (Self::Result(result_1), Self::Result(result_2)) => result_1 == result_2,
            (Self::Queue(queue_1), Self::Queue(queue_2)) => queue_1 == queue_2,
            (Self::Dict(properties_1), Self::Dict(properties_2)) => properties_1 == properties_2,
            (Self::NoneValue, Self::NoneValue) => true,
            _ => false
        }
    }
}

impl IntoPy<PyObject> for Value {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::String(string) => string.into_py(py),
            Self::Boolean(boolean) => boolean.into_py(py),
            Self::Float(number) => number.into_py(py),
            Self::Integer(number) => number.into_py(py),
            Self::Result(result) => {
                result.into_py(py)
            },
            Self::Queue(queue) => queue.to_string().into_py(py),
            Self::Dict(properties) => properties.into_py(py),
            Self::Tag(tag) => tag.into_py(py),
            Self::Array(array) => {
                let mut substrings: Vec<PyObject> = vec![];

                for value in array {
                    substrings.push(value.into_py(py));
                }

                substrings.into_py(py)
            },
            Self::NoneValue => py.None()
        }
    }
}

// -----------------------------------------------------------------------------------------------
// Dict
// -----------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Dict {
    pub properties: HashMap<String, Value>
}

impl Dict {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new()
        }
    }

    pub fn from_values(pairs: Vec<(String, Value)>) -> Self {
        let mut properties = Self::new();

        for (key, value) in pairs {
            properties.set(&key, &value);
        }

        properties
    }

    pub fn from(pairs: Vec<(String, &(dyn Any))>) -> Self {
        let mut properties = Self::new();

        for (key, value) in pairs {
            properties.set(&key, &Value::new(value));
        }

        properties
    }

    pub fn get(&self, key: &str) -> Value {
        let value = self.properties.get(key);

        value.unwrap_or(&Value::NoneValue).clone()
    }

    pub fn set(&mut self, key: &str, value: &(dyn Any)) {
        self.properties.insert(key.to_string(), Value::new(value));
    }

    pub fn extend(&mut self, properties: Dict) -> Self {
        for (key, value) in properties.properties {
            self.set(&key, &value);
        }

        self.clone()
    }

    pub fn has(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl PartialEq for Dict {
    fn eq(&self, other: &Self) -> bool {
        for (key, value) in &self.properties {
            if !other.has(key) {
                return false;
            }

            if other.get(key) != *value {
                return false;
            }
        }

        true
    }
}

impl IntoPy<PyObject> for Dict {
    fn into_py(self, py: Python) -> PyObject {
        let dict = PyDict::new(py);

        for (key, value) in self.properties {
            match value {
                Value::NoneValue => {},
                _ => {
                    dict.set_item(key, value.into_py(py)).unwrap();
                }
            }
        }

        dict.into_py(py)
    }
}