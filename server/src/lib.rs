pub mod a_star_glue;
pub mod arukone_glue;

use pyo3::prelude::*;

#[pymodule]
pub fn rust_server_glue(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<a_star_glue::PyMaze>()?;
    m.add_class::<a_star_glue::PySolution>()?;
    m.add_function(wrap_pyfunction!(a_star_glue::solve_maze, m)?)?;
    Ok(())
}
