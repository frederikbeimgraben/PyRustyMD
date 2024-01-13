// Types needed for the detection
// -----------------------------------------------------------------------------------------------

use std::{collections::HashMap, any::Any};
use crate::base::{Result, Consumable, Detector};

use pyo3::{IntoPy, Python, PyObject, types::PyDict};

// -----------------------------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------------------------

pub type Token = char;
pub type Queue = Vec<char>;

#[derive(Debug, Clone)]
pub enum Value {
    NoneValue,
    String(String),
    Boolean(bool),
    Float(f64),
    Integer(i64),
    Result(Result),
    Queue(Queue),
    Dict(Dict),
    Array(Vec<Value>)
}

impl Value {
    pub fn new(ref value: &(dyn Any)) -> Self {
        if let Some(string) = value.downcast_ref::<String>() {
            Self::String(string.clone())
        } else if let Some(boolean) = value.downcast_ref::<bool>() {
            Self::Boolean(*boolean)
        } else if let Some(number) = value.downcast_ref::<f64>() {
            Self::Float(*number)
        } else if let Some(number) = value.downcast_ref::<i64>() {
            Self::Integer(*number)
        } else if let Some(result) = value.downcast_ref::<Result>() {
            Self::Result(result.clone())
        } else if let Some(queue) = value.downcast_ref::<Queue>() {
            Self::Queue(queue.clone())
        } else if let Some(properties) = value.downcast_ref::<Dict>() {
            Self::Dict(properties.clone())
        } else if let Some(array) = value.downcast_ref::<Vec<Value>>() {
            Self::Array(array.clone())
        } else {
            // Print warning
            println!("Warning: Value::new() called with an unknown type!");

            Self::NoneValue
        }
    }

    pub fn to_str(&self) -> Option<String> {
        match self {
            Self::String(string) => Some(string.clone()),
            Self::Boolean(boolean) => Some(boolean.to_string()),
            Self::Float(number) => Some(number.to_string()),
            Self::Integer(number) => Some(number.to_string()),
            Self::Result(_) => Some(String::from("Result")),
            Self::Queue(_) => Some(String::from("Queue")),
            Self::Dict(_) => Some(String::from("Properties")),
            Self::Array(_) => Some(String::from("Array")),
            Self::NoneValue => None
        }
    }

    pub fn from(ref value: &(dyn Any)) -> Self {
        Self::new(*value)
    }

    pub fn value(&self) -> Option<&dyn Any> {
        match self {
            Self::String(string) => Some(string),
            Self::Boolean(boolean) => Some(boolean),
            Self::Float(number) => Some(number),
            Self::Integer(number) => Some(number),
            Self::Result(result) => Some(result),
            Self::Queue(queue) => Some(queue),
            Self::Dict(properties) => Some(properties),
            Self::Array(array) => Some(array),
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
                let mut properties = Dict::new();

                properties.set(
                    "attributes",
                    result.get_property("attributes")
                );

                let children: Vec<Value> = match &result.children {
                    Some(children) => children.iter().map(|child| Value::Result(child.clone())).collect(),
                    None => vec![]
                };

                properties.set(
                    "tag",
                    result.get_property("tag")
                );

                match result.detector {
                    Detector::RawDetector => {
                        result.content.unwrap_or(Queue::new()).into_py(py)
                    },
                    _ => {
                        properties.set(
                            "content",
                            Value::Array(children)
                        );

                        properties.into_py(py)
                    }
                }
            },
            Self::Queue(queue) => queue.to_string().into_py(py),
            Self::Dict(properties) => properties.into_py(py),
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

// Properties of a Result
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
            properties.set(&key, value);
        }

        properties
    }

    pub fn from(pairs: Vec<(String, &(dyn Any))>) -> Self {
        let mut properties = Self::new();

        for (key, value) in pairs {
            properties.set(&key, Value::from(value));
        }

        properties
    }

    pub fn get(&self, key: &str) -> Value {
        let value = self.properties.get(key);

        value.unwrap_or(&Value::NoneValue).clone()
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.properties.insert(key.to_string(), value);
    }

    pub fn extend(&mut self, properties: Dict) -> Self {
        for (key, value) in properties.properties {
            self.set(&key, value);
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