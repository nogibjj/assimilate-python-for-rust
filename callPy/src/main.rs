/*This is an example of calling python from Rust using O3 */

use pyo3::prelude::*;

//use the sum function from the builtins module
fn sumit() -> PyResult<()> {
    Python::with_gil(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        assert_eq!(total, 6);
        Ok(())
    })
}


fn main() {
    sumit().unwrap();
}
