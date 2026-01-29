//! Python Runtime
//! 
//! Manages the Python interpreter and script execution.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::collections::HashMap;

/// Python runtime for executing scripts
pub struct PythonRuntime {
    /// Cached script modules
    modules: HashMap<String, Py<PyModule>>,
}

impl PythonRuntime {
    /// Create a new Python runtime
    pub fn new() -> Result<Self, String> {
        // Ensure Python is initialized
        pyo3::prepare_freethreaded_python();
        
        Ok(Self {
            modules: HashMap::new(),
        })
    }
    
    /// Execute a Python script string
    pub fn execute(&self, code: &str) -> Result<(), String> {
        Python::with_gil(|py| {
            py.run(&std::ffi::CString::new(code).unwrap(), None, None)
                .map_err(|e| format!("Python error: {}", e))
        })
    }
    
    /// Execute a Python script and get a return value
    pub fn eval<T: for<'py> FromPyObject<'py>>(&self, expression: &str) -> Result<T, String> {
        Python::with_gil(|py| {
            let result = py.eval(&std::ffi::CString::new(expression).unwrap(), None, None)
                .map_err(|e| format!("Python eval error: {}", e))?;
            
            result.extract()
                .map_err(|e| format!("Python extraction error: {}", e))
        })
    }
    
    /// Load a Python module from a file
    pub fn load_module(&mut self, name: &str, code: &str) -> Result<(), String> {
        Python::with_gil(|py| {
            let module = PyModule::from_code(
                py,
                &std::ffi::CString::new(code).unwrap(),
                &std::ffi::CString::new(format!("{}.py", name)).unwrap(),
                &std::ffi::CString::new(name).unwrap(),
            ).map_err(|e| format!("Failed to load module: {}", e))?;
            
            self.modules.insert(name.to_string(), module.into());
            Ok(())
        })
    }
    
    /// Call a function in a loaded module
    pub fn call_function<T: for<'py> FromPyObject<'py>>(
        &self,
        module_name: &str,
        function_name: &str,
        args: Vec<PyArg>,
    ) -> Result<T, String> {
        let module = self.modules.get(module_name)
            .ok_or_else(|| format!("Module '{}' not loaded", module_name))?;
        
        Python::with_gil(|py| {
            let module = module.bind(py);
            let func = module.getattr(function_name)
                .map_err(|e| format!("Function not found: {}", e))?;
            
            let py_args: Vec<PyObject> = args.iter()
                .map(|arg| arg.to_py(py))
                .collect();
            
            let result = func.call1((py_args.into_iter().collect::<Vec<_>>(),))
                .map_err(|e| format!("Call failed: {}", e))?;
            
            result.extract()
                .map_err(|e| format!("Extraction failed: {}", e))
        })
    }
    
    /// Set a global variable accessible to scripts
    pub fn set_global(&self, name: &str, value: PyArg) -> Result<(), String> {
        Python::with_gil(|py| {
            let globals = PyDict::new(py);
            globals.set_item(name, value.to_py(py))
                .map_err(|e| format!("Failed to set global: {}", e))
        })
    }
    
    /// Check if Python is available
    pub fn is_available() -> bool {
        Python::with_gil(|_| true)
    }
    
    /// Get Python version
    pub fn version() -> String {
        Python::with_gil(|py| {
            py.version().to_string()
        })
    }
}

/// Python argument types
#[derive(Debug, Clone)]
pub enum PyArg {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<PyArg>),
    Dict(HashMap<String, PyArg>),
    None,
}

impl PyArg {
    pub fn to_py(&self, py: Python<'_>) -> PyObject {
        match self {
            PyArg::Int(v) => v.into_pyobject(py).unwrap().into_any().unbind(),
            PyArg::Float(v) => v.into_pyobject(py).unwrap().into_any().unbind(),
            PyArg::String(v) => v.into_pyobject(py).unwrap().into_any().unbind(),
            PyArg::Bool(v) => v.into_pyobject(py).unwrap().into_any().unbind(),
            PyArg::List(v) => {
                let list: Vec<PyObject> = v.iter().map(|a| a.to_py(py)).collect();
                list.into_pyobject(py).unwrap().into_any().unbind()
            }
            PyArg::Dict(v) => {
                let dict = PyDict::new(py);
                for (k, val) in v {
                    dict.set_item(k, val.to_py(py)).unwrap();
                }
                dict.into_any().unbind()
            }
            PyArg::None => py.None(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_execute() {
        let rt = PythonRuntime::new().unwrap();
        assert!(rt.execute("x = 1 + 1").is_ok());
    }
    
    #[test]
    fn test_python_eval() {
        let rt = PythonRuntime::new().unwrap();
        let result: i64 = rt.eval("1 + 2").unwrap();
        assert_eq!(result, 3);
    }
}
