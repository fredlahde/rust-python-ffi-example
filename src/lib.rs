#![allow(non_snake_case)]

#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};

py_class!(pub class MyType |py| {
    data number: i32;
    def __new__(_cls, arg: i32) -> PyResult<MyType> {
        MyType::create_instance(py, arg)
    }
    def n(&self) -> PyResult<i32> {
        Ok(*self.number(py))
    }
});

py_class!(pub class Properties |py| {
    data s: String;
    data skip: usize;
    def __new__(_cls, s: String, skip: usize) -> PyResult<Properties> {
        Properties::create_instance(py, s, skip)
    }
});

fn count_doubles(py: Python, props: Properties) -> PyResult<MyType> {
    let mut total = 0u64;

    let chars = props.s(py).chars();
    let chars2 = chars.clone();
    let skip = *props.skip(py);
    for (c1, c2) in chars.zip(chars2.skip(skip)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(MyType::create_instance(py, total as i32)?)
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(
        py,
        "count_doubles",
        py_fn!(py, count_doubles(props: Properties)),
    )?;
    m.add_class::<MyType>(py)?;
    m.add_class::<Properties>(py)?;
    Ok(())
});
