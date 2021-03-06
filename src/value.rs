//! The `Value` Python object to build WebAssembly values.

use cpython::{PyObject, PyResult, Python, PythonObject, ToPyObject};
use wasmer_runtime::Value as WasmValue;

/// The `Value` Python object represents a WebAssembly value.
///
/// # Examples
///
/// ```python,ignore
/// from wasmer import Value
///
/// value1 = Value.from_i32(42)
/// value2 = Value.from_i64(42)
/// value3 = Value.from_f32(4.2)
/// value4 = Value.from_f64(4.2)
///
/// print(repr(value1)) // "I32(42)"
/// print(repr(value2)) // "I64(42)"
/// print(repr(value3)) // "F32(4.2)"
/// print(repr(value4)) // "F64(4.2)"
/// ```
py_class!(pub class Value |py| {
    data value: WasmValue;

    @staticmethod
    def i32(value: i32) -> PyResult<Value> {
        Value::create_instance(
            py,
            WasmValue::I32(value)
        )
    }

    @staticmethod
    def i64(value: i64) -> PyResult<Value> {
        Value::create_instance(
            py,
            WasmValue::I64(value)
        )
    }

    @staticmethod
    def f32(value: f32) -> PyResult<Value> {
        Value::create_instance(
            py,
            WasmValue::F32(value)
        )
    }

    @staticmethod
    def f64(value: f64) -> PyResult<Value> {
        Value::create_instance(
            py,
            WasmValue::F64(value)
        )
    }

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.value(py)))
    }
});

/// Getter to access the private `value` attribute of the `Value` Python object.
pub(crate) fn get_wasm_value(py: Python, value: &Value) -> WasmValue {
    value.value(py).clone()
}

/// Transform a `WasmValue` into a `PyObject`.
pub(crate) fn wasm_value_into_python_object(py: Python, wasm_value: &WasmValue) -> PyObject {
    match wasm_value {
        WasmValue::I32(value) => value.into_py_object(py).into_object(),
        WasmValue::I64(value) => value.into_py_object(py).into_object(),
        WasmValue::F32(value) => value.into_py_object(py).into_object(),
        WasmValue::F64(value) => value.into_py_object(py).into_object(),
    }
}
