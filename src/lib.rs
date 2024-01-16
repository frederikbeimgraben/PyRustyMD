// Python Bindings
// ------------------------

pub mod base;
pub mod types;
pub mod detectors;
pub mod advanced_detectors;
pub mod html;
pub mod md;

use html::HTMLDetector;
use pyo3::prelude::*;

use types::{Queue, Value};
use crate::base::*;

#[pyfunction]
fn parse(input: &str) -> PyResult<PyObject> {
    let mut consumable: Queue = Consumable::from_string(input.to_string());

    let result = consumable.consume_any(
        &vec![
            Detector::HTMLDetector(HTMLDetector::DivDetector),
            Detector::HTMLDetector(HTMLDetector::ParagraphDetector),
            Detector::HTMLDetector(HTMLDetector::ImgDetector),
            Detector::HTMLDetector(HTMLDetector::LinkDetector),
            Detector::HTMLDetector(HTMLDetector::HeadingDetector),
            Detector::HTMLDetector(HTMLDetector::SpanDetector)
        ]
    );

    let result_value = match result {
        Some(result) => Value::Array(
            result.iter().map(|result| Value::Result(result.clone())).collect::<Vec<Value>>()
        ),
        None => Value::NoneValue
    };

    let py_gil = Python::acquire_gil();

    let py = py_gil.python();

    Ok(result_value.into_py(py))
}

#[pymodule]
fn pyrustymd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;

    Ok(())
}