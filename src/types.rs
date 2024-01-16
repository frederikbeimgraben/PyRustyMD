// Types needed for the detection
// -----------------------------------------------------------------------------------------------

use std::{collections::HashMap, any::Any, fmt::Debug, fmt::Display};

use pyo3::{IntoPy, Python, PyObject, types::PyDict};

use crate::tag::Tag;

// ===============================================================================================
// Types
// ===============================================================================================

pub trait QueueTrait: Any + Debug + Clone {
    fn consume(&mut self, detector: &(dyn Detectable)) -> Option<(String, Tag)>;
    fn next(&mut self) -> Option<char>;
    fn consume_all(&mut self, detectors: Vec<&(dyn Detectable)>) -> Tag;
}

pub type Queue = String;

impl QueueTrait for Queue {
    fn consume(&mut self, detector: &(dyn Detectable)) -> Option<(String, Tag)> {
        let mut queue = self.clone();

        let result = detector.detect(&mut queue);

        let consumed: usize = self.len() - queue.len();

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

    fn consume_all(&mut self, detectors: Vec<&(dyn Detectable)>) -> Tag {
        let mut results: Vec<Tag> = vec![];

        let mut buffer: String = String::new();

        loop {
            // Get the first result of the detectors which is not None
            let mut first_result: Option<(Queue, Tag)> = None;

            let mut copy = self.clone();

            for detector in &detectors {
                copy = self.clone();

                first_result = copy.consume(*detector);

                if first_result.is_some() {
                    break;
                }
            }

            match first_result {
                Some(mut result) => {
                    // Unpack buffer and push it to the results
                    if buffer.len() > 0 {
                        let buffer_result = Tag::new(
                            "raw".to_string(),
                            None,
                            &buffer,
                        );

                        results.push(buffer_result);

                        buffer = String::new();
                    }

                    let consumed = self.len() - copy.len();

                    // Remove the consumed characters from the queue
                    self.drain(0..consumed);

                    if result.0.len() > 0 {
                        // Push the result to the results
                        let inner_results = result.0.clone().consume_all(detectors.clone());

                        result.1.children = Some(inner_results.children.unwrap_or(vec![]));

                        results.push(result.1);
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
                                let buffer_result = Tag::new(
                                    "raw".to_string(),
                                    None,
                                    &buffer,
                                );

                                results.push(buffer_result);
                            }

                            break;
                        }
                    }

                    // Remove the first character of the queue
                    self.drain(0..1);
                }
            }
        }

        let result = Tag::new(
            "root".to_string(),
            None,
            &results,
        );

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

// -----------------------------------------------------------------------------------------------
// Detectable
// -----------------------------------------------------------------------------------------------

pub trait Detectable: Debug {
    fn detect(&self, queue: &mut Queue) -> Option<(Queue, Tag)>;
    fn regex(&self, _: &mut Queue) -> String {
        String::new()
    }
}

#[derive(Debug)]
pub enum Detector<'a> {
    Any(&'a(dyn Detectable)),
    None
}

impl Detectable for Detector<'_> {
    fn detect(&self, queue: &mut Queue) -> Option<(Queue, Tag)> {
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
            value if value.is::<Dict>()       => Self::Dict(value.downcast_ref::<Dict>().unwrap().clone()),
            value if value.is::<Vec<Value>>() => Self::Array(value.downcast_ref::<Vec<Value>>().unwrap().clone()),
            value if value.is::<Tag>()        => Self::Tag(value.downcast_ref::<Tag>().unwrap().clone()),

            // &str
            value if value.is::<&str>() => {
                let value = value.downcast_ref::<&str>().unwrap();

                Self::String(value.to_string())
            },

            // Array of Tags
            value if value.is::<Vec<Tag>>() => {
                let input_array = value.downcast_ref::<Vec<Tag>>().unwrap();

                let mut array: Vec<Value> = vec![];

                for v in input_array {
                    array.push(Self::Tag(v.clone()));
                }

                Self::Array(array)
            },

            // Array of Values
            value if value.is::<Vec<Value>>() => {
                let input_array = value.downcast_ref::<Vec<Value>>().unwrap();

                let mut array: Vec<Value> = vec![];

                for v in input_array {
                    array.push(v.clone());
                }

                Self::Array(array)
            },

            // Value
            value if value.is::<Value>() => value.downcast_ref::<Value>().unwrap().clone(),

            // Option
            value if value.is::<Option<&(dyn Any)>>() => {
                let value = value.downcast_ref::<Option<&(dyn Any)>>().unwrap();

                match value {
                    Some(value) => Self::new(value),
                    None => Self::NoneValue
                }
            },

            // Any other type
            _ => Self::NoneValue
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Primitive Types
            Self::String(string) => write!(f, "{}", string),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Float(number) => write!(f, "{}", number),
            Self::Integer(number) => write!(f, "{}", number),

            // Application Types
            Self::Queue(queue) => write!(f, "{}", queue),
            Self::Tag(_) => write!(f, "Tag(...)"),

            // Collections
            Self::Dict(_) => write!(f, "{{...}}"),
            Self::Array(array) => {
                let mut substrings: Vec<String> = vec![];

                for value in array {
                    substrings.push(format!("{}", value));
                }

                write!(f, "[{}]", substrings.join(", "))
            },

            // None
            Self::NoneValue => write!(f, "None")
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

    pub fn get(&self, key: &str) -> Option<Value> {
        let value = self.properties.get(key);

        match value {
            Some(value) => Some(value.clone()),
            None => None
        }
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

    pub fn remove(&mut self, key: &str) {
        self.properties.remove(key);
    }

    // Iter
    pub fn iter(&self) -> Dict {
        self.clone()
    }
}

impl PartialEq for Dict {
    fn eq(&self, other: &Self) -> bool {
        for (key, value) in &self.properties {
            if !other.has(key) {
                return false;
            }

            if other.get(key).unwrap_or(Value::NoneValue) != *value {
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

impl Iterator for Dict {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        let properties = self.properties.clone();

        let (key, value) = properties.iter().next().unwrap();

        self.properties.remove(key);

        Some((key.clone(), value.clone()))
    }
}