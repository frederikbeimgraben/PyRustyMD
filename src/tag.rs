// =====================
// Detect a single tag
// =====================

use crate::types::*;

use pyo3::{IntoPy, Python, PyObject};

// Tag
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    pub name: String,
    pub attributes: Dict,
    pub children: Vec<Tag>,
}

impl IntoPy<PyObject> for Tag {
    fn into_py(self, py: Python) -> PyObject {
        let mut tag = Dict::new();

        tag.set("name", &self.name);
        tag.set("attributes", &self.attributes);
        tag.set("content", &self.children);

        tag.into_py(py)
    }
}