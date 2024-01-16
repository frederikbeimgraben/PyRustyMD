// Python Bindings
// ------------------------

pub mod types;
pub mod tag;
pub mod tags;

use pyo3::prelude::*;

use tags::tag_scope_detector::TagScopeDetector;
use types::{Queue, QueueTrait};

#[pyfunction]
fn parse(input: &str) -> PyResult<PyObject> {
    let mut consumable: Queue = Queue::from(input);

    let result = consumable.consume_all(
        vec![
            &TagScopeDetector::new(None, None, None, None, None),
        ]
    );

    let py_gil = Python::acquire_gil();

    let py = py_gil.python();

    Ok(result.into_py(py))
}

#[pymodule]
fn pyrustymd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;

    Ok(())
}