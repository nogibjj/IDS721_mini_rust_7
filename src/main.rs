use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}

// use rustpython_vm::pyobject::{PyResult, PyObjectRef};
// use rustpython_vm::compile;

// fn main() -> PyResult<()> {
//     let code = "print('Hello, RustPython!')";
//     let code_obj = compile::compile(&code, "test.py".to_string(), "exec".to_string(), 0)?;
//     let globals = rustpython_vm::pyobject::new_scope(None);
//     rustpython_vm::eval::eval_module(code_obj.clone(), globals.clone())?;
//     Ok(())
// }

// lib.rs

// use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

// #[pyfunction]
// fn greet(name: &str) -> PyResult<String> {
//     Ok(format!("Hello, {}!", name))
// }

// #[pymodule]
// fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(greet, m)?)?;

//     Ok(())
// }
