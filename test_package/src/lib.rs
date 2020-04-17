use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::time;

#[pyfunction]
fn make_output(
    nvars: usize,
    timesteps: usize,
    copies: usize,
) -> PyResult<Vec<(Vec<Vec<bool>>, f64)>> {
    let sec = time::Duration::from_millis(3000);
    std::thread::sleep(sec);

    let result = Ok((0..copies)
        .map(|_| {
            let vecs = (0..timesteps)
                .map(|_| vec![false; nvars])
                .collect::<Vec<_>>();
            (vecs, 0.0)
        })
        .collect());

    std::thread::sleep(sec);

    result
}

#[pyfunction]
fn make_simple_output(entries: usize) -> PyResult<Vec<bool>> {
    let sec = time::Duration::from_millis(3000);
    std::thread::sleep(sec);

    let result = Ok(vec![false; entries]);

    std::thread::sleep(sec);

    result
}

#[pymodule]
fn pyo3_check(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(make_output))?;
    m.add_wrapped(wrap_pyfunction!(make_simple_output))?;
    Ok(())
}
