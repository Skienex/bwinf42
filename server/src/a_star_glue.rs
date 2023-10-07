use a_star::Maze;

use pyo3::prelude::*;

pub const CELL_EMPTY: usize = 0;
pub const CELL_WALL: usize = 1;
pub const CELL_START: usize = 2;
pub const CELL_GOAL: usize = 3;

#[pyclass(name = "Solution")]
pub struct PySolution {
    #[pyo3(get, set)]
    pub maze: PyMaze,
    #[pyo3(get, set)]
    pub path: Vec<(i16, i16, i16)>,
}

#[pyclass(name = "Maze")]
#[derive(Clone)]
pub struct PyMaze {
    #[pyo3(get, set)]
    pub dimensions: (usize, usize, usize),
    #[pyo3(get, set)]
    pub cells: Vec<Vec<Vec<usize>>>,
}

#[pyfunction]
pub fn solve_maze(source: String) -> PyResult<Option<PySolution>> {
    let Some(mut maze) = Maze::parse(&source) else {
        return Ok(None);
    };
    let [dx, dy, dz] = maze.dimensions;
    let path = maze
        .a_star()
        .unwrap_or_default()
        .into_iter()
        .map(|[x, y, z]| (x, y, z))
        .collect();
    let maze = PyMaze {
        dimensions: (dx, dy, dz),
        cells: maze
            .cells
            .into_iter()
            .map(|it| {
                it.into_iter()
                    .map(|it| it.into_iter().map(|it| it.typ as usize).collect())
                    .collect()
            })
            .collect(),
    };
    Ok(Some(PySolution { maze, path }))
}
