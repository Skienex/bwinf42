use checker::{start_checker, tokio};
use pyo3::prelude::*;
use std::sync::mpsc;

#[pyclass(name = "Grid")]
pub struct PyGrid {
    #[pyo3(get, set)]
    pub num_count: u16,
    #[pyo3(get, set)]
    pub cells: Vec<Vec<u16>>,
}

#[pyfunction]
pub fn generate_arukone(n: u16) -> PyResult<Option<PyGrid>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { async_generate_arukone(n).await })
}

async fn async_generate_arukone(n: u16) -> PyResult<Option<PyGrid>> {
    let (grid_sender, grid_receiver) = mpsc::channel();
    let (output_sender, output_reveicer) = mpsc::channel();
    let handle = tokio::spawn(async { start_checker(grid_receiver, output_sender).await? });
    todo!()
}
