// JSON Types
// ----------------------------------

use std::{collections::HashMap, any::Any};
use crate::base::{Result, Consumable, Detector};

use pyo3::{IntoPy, Python, PyObject, types::PyDict};

// -----------------------------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------------------------

pub type Token = char;
pub type Queue = Vec<char>;

// -----------------------------------------------------------------------------------------------
// JSON Trait
// -----------------------------------------------------------------------------------------------

pub trait JSON {
    fn to_json(&self) -> String;
    fn to_value(&self) -> Value {
        Value::String(self.to_json())
    }
}

// -----------------------------------------------------------------------------------------------
// Supertraits
// -----------------------------------------------------------------------------------------------

// Saves a String, a boolean, a number or a Result
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

impl JSON for Value {
    fn to_json(&self) -> String {
        match self {
            Self::String(string) => format!("\"{}\"", string),
            Self::Boolean(boolean) => boolean.to_string(),
            Self::Float(number) => number.to_string(),
            Self::Integer(number) => number.to_string(),
            Self::Result(result) => result.to_json(),
            Self::Queue(queue) => queue.to_json(),
            Self::Dict(properties) => properties.to_json(),
            Self::Array(array) => {
                let substrings: Vec<String> = array.iter().map(|value| value.to_json()).collect();

                format!("[{}]", substrings.join(","))
            },
            Self::NoneValue => String::from("null")
        }
    }

    fn to_value(&self) -> Value {
        self.clone()
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

                if let Some(det_properties) = &result.properties {
                    for (key, value) in det_properties.clone().properties {
                        if key == "attributes" {
                            continue;
                        }
                        properties.set(&key, value);
                    }

                    match det_properties.get("attributes") {
                        Value::Dict(attributes) => {
                            for (key, value) in attributes.properties {
                                if key == "class" || key == "id" || key == "style" {
                                    continue;
                                }

                                properties.set(&key, value);
                            }
                        },
                        _ => {}
                    }
                }

                let children: Vec<Value> = match &result.children {
                    Some(children) => children.iter().map(|child| Value::Result(child.clone())).collect(),
                    None => vec![]
                };

                properties.set(
                    "children",
                    Value::Array(children)
                );

                match result.detector {
                    Detector::RawDetector => {
                        return result.content.unwrap_or(Queue::new()).to_string().into_py(py);
                    },
                    _ => {
                        properties.into_py(py)
                    }
                }
            },
            Self::Queue(queue) => queue.into_py(py),
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

impl JSON for Dict {
    fn to_json(&self) -> String {
        let mut substrings: Vec<String> = vec![];

        for (key, value) in &self.properties {
            substrings.push(format!("\"{}\":{}", key, value.to_json()));
        }

        format!("{{{}}}", substrings.join(","))
    }

    fn to_value(&self) -> Value {
        Value::Dict(self.clone())
    }
}

impl IntoPy<PyObject> for Dict {
    fn into_py(self, py: Python) -> PyObject {
        let dict = PyDict::new(py);

        for (key, value) in self.properties {
            dict.set_item(key, value.into_py(py)).unwrap();
        }

        dict.into_py(py)
    }
}

// -----------------------------------------------------------------------------------------------

impl JSON for Queue {
    fn to_json(&self) -> String {
        // Stringify the queue
        format!(
            "\"{}\"",
            self.to_string()
        )
    }

    fn to_value(&self) -> Value {
        Value::Queue(self.clone())
    }
}

impl JSON for Result {
    fn to_json(&self) -> String {
        // First convert to a Dict
        let mut properties = Dict::new();

        if let Some(det_properties) = &self.properties {
            properties.set("properties", Value::from(det_properties));
        }

        let children: Vec<Value> = match &self.children {
            Some(children) => children.iter().map(|child| Value::Result(child.clone())).collect(),
            None => vec![]
        };

        let content = match &self.content {
            Some(content) => Value::Queue(content.clone()),
            None => Value::NoneValue
        };

        match self.detector {
            Detector::RawDetector => {
                return content.to_json();
            },
            _ => {
                properties.set(
                    "children",
                    Value::Array(children)
                );
            }
        }

        // Then stringify the Dict
        properties.to_json()
    }

    fn to_value(&self) -> Value {
        Value::Result(self.clone())
    }
}

impl JSON for Option<Vec<Result>> {
    fn to_json(&self) -> String {
        match self {
            Some(results) => {
                let mut substrings: Vec<String> = vec![];

                for result in results {
                    substrings.push(result.to_json());
                }

                format!("[{}]", substrings.join(","))
            },
            None => String::from("null")
        }
    }

    fn to_value(&self) -> Value {
        match self {
            Some(results) => {
                let mut substrings: Vec<Value> = vec![];

                for result in results {
                    substrings.push(result.to_value());
                }

                Value::Array(substrings)
            },
            None => Value::NoneValue
        }
    }
}