use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::wrap_pyfunction;

use crate::sentience_core::{
    ast::{SentienceTokenAst, ThoughtType, Value},
    runtime::{ExecutionResult, SimpleRuntime},
    SentienceCore,
};

/// Python wrapper for Sentience Core
#[pyclass]
pub struct PySentienceCore {
    core: SentienceCore,
}

#[pymethods]
impl PySentienceCore {
    #[new]
    fn new() -> Self {
        let runtime = Box::new(SimpleRuntime::new());
        let core = SentienceCore::new(runtime);
        Self { core }
    }

    /// Parse Sentience DSL into AST
    fn parse(&self, src: &str) -> PyResult<PySentienceTokenAst> {
        let ast = self
            .core
            .parse(src)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        Ok(PySentienceTokenAst { ast })
    }

    /// Canonicalize AST for deterministic processing
    fn canonicalize(&self, ast: &PySentienceTokenAst) -> PySentienceTokenAst {
        let canon = self.core.canonicalize(&ast.ast);
        PySentienceTokenAst { ast: canon }
    }

    /// Generate deterministic token ID
    fn hash(&self, canon: &PySentienceTokenAst) -> String {
        self.core.hash(&canon.ast)
    }

    /// Generate embedding vector
    fn embed(&self, canon: &PySentienceTokenAst) -> Vec<f32> {
        self.core.embed(&canon.ast)
    }

    /// Execute AST and return results
    fn execute(&mut self, ast: &PySentienceTokenAst) -> PyResult<PyExecutionResult> {
        let result = self
            .core
            .execute(&ast.ast)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        Ok(PyExecutionResult { result })
    }

    /// Complete pipeline: parse → canonicalize → hash → embed → execute
    fn process_step(&mut self, src: &str) -> PyResult<PyExecutionResult> {
        let result = self
            .core
            .process_step(src)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        Ok(PyExecutionResult { result })
    }
}

/// Python wrapper for SentienceTokenAst
#[pyclass]
pub struct PySentienceTokenAst {
    ast: SentienceTokenAst,
}

#[pymethods]
impl PySentienceTokenAst {
    /// Get token type as string
    fn token_type(&self) -> String {
        self.ast.ttype.to_string()
    }

    /// Get field value by key
    fn get_field(&self, key: &str) -> Option<String> {
        self.ast.get_field_str(key).map(|s| s.to_string())
    }

    /// Convert to Python dict
    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);

        dict.set_item("type", self.ast.ttype.to_string())?;

        let fields = PyDict::new_bound(py);
        for field in &self.ast.fields {
            fields.set_item(&field.key, value_to_python(&field.value, py)?)?;
        }
        dict.set_item("fields", fields)?;

        let span = PyDict::new_bound(py);
        span.set_item("line", self.ast.span.line)?;
        span.set_item("col", self.ast.span.col)?;
        span.set_item("end_line", self.ast.span.end_line)?;
        span.set_item("end_col", self.ast.span.end_col)?;
        dict.set_item("span", span)?;

        Ok(dict.into())
    }
}

/// Python wrapper for ExecutionResult
#[pyclass]
pub struct PyExecutionResult {
    result: ExecutionResult,
}

#[pymethods]
impl PyExecutionResult {
    /// Get token ID
    fn token_id(&self) -> Option<String> {
        self.result.token_id.clone()
    }

    /// Get embedding as numpy array
    fn embedding(&self, py: Python) -> Option<PyObject> {
        self.result.embedding.as_ref().map(|emb| {
            let array = PyArray1::from_slice_bound(py, emb);
            array.into()
        })
    }

    /// Get RefNet metrics
    fn metrics(&self, py: Python) -> Option<PyObject> {
        self.result.metrics.as_ref().map(|metrics| {
            let dict = PyDict::new_bound(py);
            dict.set_item("valence", metrics.valence).unwrap();
            dict.set_item("smd", metrics.smd).unwrap();
            dict.set_item("quality", metrics.quality).unwrap();
            dict.set_item("next_action", &metrics.next_action).unwrap();

            let logits = PyDict::new_bound(py);
            for (action, score) in &metrics.action_logits {
                logits.set_item(action, score).unwrap();
            }
            dict.set_item("action_logits", logits).unwrap();

            dict.into()
        })
    }

    /// Get generated tokens
    fn tokens(&self, py: Python) -> PyResult<PyObject> {
        let list = PyList::new_bound(py, Vec::<PyObject>::new());
        for token in &self.result.tokens {
            let token_dict = PyDict::new_bound(py);
            token_dict.set_item("id", &token.id)?;
            token_dict.set_item("type", token.ast.ttype.to_string())?;

            let fields = PyDict::new_bound(py);
            for field in &token.ast.fields {
                fields.set_item(&field.key, value_to_python(&field.value, py)?)?;
            }
            token_dict.set_item("fields", fields)?;

            let meta = PyDict::new_bound(py);
            meta.set_item("version", &token.meta.version)?;
            meta.set_item("strength", token.meta.strength)?;
            meta.set_item("belief", token.meta.belief)?;
            meta.set_item("tags", &token.meta.tags)?;
            token_dict.set_item("meta", meta)?;

            list.append(token_dict)?;
        }
        Ok(list.into())
    }

    /// Get generated edges
    fn edges(&self, py: Python) -> PyResult<PyObject> {
        let list = PyList::new_bound(py, Vec::<PyObject>::new());
        for edge in &self.result.edges {
            let edge_dict = PyDict::new_bound(py);
            edge_dict.set_item("id", &edge.id)?;
            edge_dict.set_item("source_id", &edge.source_id)?;
            edge_dict.set_item("target_id", &edge.target_id)?;
            edge_dict.set_item("edge_type", edge.edge_type.to_string())?;
            edge_dict.set_item("weight", edge.weight)?;
            edge_dict.set_item("timestamp", edge.timestamp)?;
            list.append(edge_dict)?;
        }
        Ok(list.into())
    }
}

/// Convert Rust Value to Python object
fn value_to_python(value: &Value, py: Python) -> PyResult<PyObject> {
    match value {
        Value::Str(s) => Ok(s.into_py(py)),
        Value::Num(n) => Ok(n.into_py(py)),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Path(path) => {
            let list = PyList::new_bound(py, path);
            Ok(list.into())
        }
        Value::List(list) => {
            let py_list = PyList::new_bound(py, Vec::<PyObject>::new());
            for item in list {
                py_list.append(value_to_python(item, py)?)?;
            }
            Ok(py_list.into())
        }
        Value::Map(map) => {
            let dict = PyDict::new_bound(py);
            for (key, val) in map {
                dict.set_item(key, value_to_python(val, py)?)?;
            }
            Ok(dict.into())
        }
    }
}

/// Create a new Sentience Core instance
#[pyfunction]
fn create_sentience_core() -> PySentienceCore {
    PySentienceCore::new()
}

/// Python module definition
#[pymodule]
fn sentience_core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySentienceCore>()?;
    m.add_class::<PySentienceTokenAst>()?;
    m.add_class::<PyExecutionResult>()?;
    m.add_function(wrap_pyfunction!(create_sentience_core, m)?)?;

    // Add token types as constants
    m.add("THOUGHT_TYPE_PERCEPT", ThoughtType::Percept.to_string())?;
    m.add(
        "THOUGHT_TYPE_REFLECTION",
        ThoughtType::Reflection.to_string(),
    )?;
    m.add("THOUGHT_TYPE_ACTION", ThoughtType::Action.to_string())?;
    m.add("THOUGHT_TYPE_CONCEPT", ThoughtType::Concept.to_string())?;
    m.add(
        "THOUGHT_TYPE_SELF_MODEL",
        ThoughtType::SelfModel.to_string(),
    )?;

    Ok(())
}
