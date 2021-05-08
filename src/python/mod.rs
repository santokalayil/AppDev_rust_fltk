// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;

// pub fn show() -> Result<(), ()> {
//     Python::with_gil(|py| {
//         main_(py).map_err(|e| {
//           // We can't display Python exceptions via std::fmt::Display,
//           // so print the error here manually.
//           e.print_and_set_sys_last_vars(py);
//         })
//     })
// }

// fn main_(py: Python) -> PyResult<()> {
//     let sys = py.import("py_module")?;
//     let version: String = sys.get("string")?.extract()?;
//     let locals = [("os", py.import("os")?)].into_py_dict(py);
//     let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//     let user: String = py.eval(code, None, Some(&locals))?.extract()?;
//     println!("Hello {}, I'm Python {}", user, version);
//     Ok(())
// }

// use pyo3::prelude::*;

// pub fn print_from_python_module() -> PyResult<()> {
//     Python::with_gil(|py| {
//         println!("HI GOOD MORNING");
//         let py_module = PyModule::import(py, "py_module")?;
//         println!("HI GOOD MORNING");
//         // let total: i32 = builtins.getattr("sum")?.call1((vec![1, 2, 3],))?.extract()?;
//         // assert_eq!(total, 6);
//         let string: String = py_module.get("string")?.extract()?;
//         println!("Module imported string is {}", string);
//         Ok(())
//     })
// }
