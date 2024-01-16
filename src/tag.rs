// =====================
// Detect a single tag
// =====================

use std::any::Any;

use crate::types::*;

use pyo3::{IntoPy, Python, PyObject};

// Tag
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    pub name: String,
    pub attributes: Dict,
    pub children: Option<Vec<Tag>>,
    pub content: Option<String>,
    is_raw: bool,
}

impl Tag {
    pub fn init(name: String, attributes: Option<Dict>) -> Self {
        let attributes = match attributes {
            Some(attributes) => attributes,
            None => Dict::new(),
        };

        Self {
            name,
            attributes,
            children: None,
            content: None,
            is_raw: false,
        }
    }

    pub fn new(name: String, attributes: Option<Dict>, content: &(dyn Any)) -> Self {
        let attributes = match attributes {
            Some(attributes) => attributes,
            None => Dict::new(),
        };

        let mut raw = false;

        let children = match content.downcast_ref::<Vec<Tag>>() {
            Some(children) => {
                Some(children.clone())
            }
            None => None,
        };

        let content = match content.downcast_ref::<String>() {
            Some(content) => {
                raw = true;

                Some(content.clone())
            }
            None => None,
        };

        Self {
            name,
            attributes,
            children,
            content,
            is_raw: raw,
        }
    }

    // Attributes
    pub fn has_attributes(&self) -> bool {
        self.attributes.len() > 0
    }

    pub fn has(&self, key: &str) -> bool {
        self.attributes.has(key)
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.attributes.get(key)
    }

    pub fn set(&mut self, key: &str, value: &(dyn Any)) {
        self.attributes.set(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.attributes.remove(key);
    }

    pub fn is_raw(&self) -> bool {
        self.is_raw
    }

    pub fn has_children(&self) -> bool {
        match &self.children {
            Some(children) => children.len() > 0,
            None => false,
        }
    }

    pub fn has_content(&self) -> bool {
        match &self.content {
            Some(content) => content.len() > 0,
            None => false,
        }
    }
}

impl IntoPy<PyObject> for Tag {
    fn into_py(self, py: Python) -> PyObject {
        let mut tag = Dict::new();

        tag.set("name", &self.name);
        tag.set("attributes", &self.attributes);

        match self.children {
            Some(children) => {
                tag.set("content", &children);
            }
            None => {
                tag.set("content", &self.content.unwrap_or(String::new()));
            }
        }

        tag.into_py(py)
    }
}