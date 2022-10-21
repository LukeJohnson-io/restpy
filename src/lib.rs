use std::cmp::min;

use pyo3::prelude::*;


#[pyclass]
struct Binomial {
    trials: u128,
    prob: f64,
}

#[pymethods]
impl Binomial {
    #[new]
    fn new(trials: u128, prob: f64) -> Self {
        Self {
            trials: trials,
            prob: prob,
        }
    }

    fn dist(&self, successes: u128) -> f64 {
        comb(self.trials, successes)
        * self.prob.powi(successes as i32)
        * (1.0_f64 - self.prob).powi((self.trials - successes) as i32)
    }
}

#[pyfunction]
fn comb(n: u128, k: u128) -> f64 {
    if n == k {
        return 1.0;
    }
    let upper: u128 = min(k, n-k);
    let pool = n as f64;
    (1..=upper).into_iter().map(|i| (pool + 1.0 - i as f64)/(i as f64)).product::<f64>()
}

/// A Python module implemented in Rust.
#[pymodule]
fn restpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(comb, m)?)?;
    m.add_class::<Binomial>()?;
    Ok(())
}
