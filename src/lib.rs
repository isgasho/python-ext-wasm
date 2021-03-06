#![deny(warnings)]

#[macro_use]
extern crate cpython;

use cpython::PyBytes;
use std::{ops::Deref, thread};

mod error;
mod instance;
mod memory_view;
mod value;

use instance::{validate, Instance};
use value::Value;

/// A `Shell` is a thread-safe wrapper over a value `T` that will fail
/// if used in another thread. Why? All data used by Python must be
/// thread-safe. However some WebAssembly data cannot be thread-safe,
/// like unshared memory. With a `Shell`, the program will compile and
/// Python will be able to use the value `T`, but it must not be
/// passed between threads. The documentation will specify it.
pub struct Shell<T> {
    /// The thread ID where the datum has been created.
    thread_id: thread::ThreadId,

    /// The datum.
    value: T,
}

impl<T> Shell<T> {
    pub fn new(value: T) -> Self {
        Self {
            thread_id: thread::current().id(),
            value,
        }
    }
}

/// A `Shell` is sendable.
unsafe impl<T> Send for Shell<T> {}

/// Dereferences the value if it's inside the same thread than the
/// creation thread.
impl<T> Deref for Shell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if thread::current().id() != self.thread_id {
            panic!("The current `Shell` cannot be dereferenced in a different thread.");
        }

        &self.value
    }
}

// Declare the module.
py_module_initializer!(libwasmer, initlibwasmer, PyInit_wasmer, |python, module| {
    module.add(
        python,
        "__doc__",
        "This extension allows to manipulate and to execute WebAssembly binaries.",
    )?;
    module.add_class::<memory_view::Uint8MemoryView>(python)?;
    module.add_class::<memory_view::Int8MemoryView>(python)?;
    module.add_class::<memory_view::Uint16MemoryView>(python)?;
    module.add_class::<memory_view::Int16MemoryView>(python)?;
    module.add_class::<memory_view::Uint32MemoryView>(python)?;
    module.add_class::<memory_view::Int32MemoryView>(python)?;
    module.add_class::<Instance>(python)?;
    module.add_class::<Value>(python)?;
    module.add(python, "validate", py_fn!(python, validate(bytes: PyBytes)))?;

    Ok(())
});
