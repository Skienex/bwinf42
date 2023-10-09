use checker::{
    start_checker,
    tokio::{self, runtime::Handle},
};
use generator::{CellType, Grid};
use pyo3::prelude::*;
use std::{
    hint::spin_loop,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

#[pyclass(name = "Grid")]
pub struct PyGrid {
    #[pyo3(get, set)]
    pub num_count: u16,
    #[pyo3(get, set)]
    pub cells: Vec<Vec<u16>>,
}

#[pyfunction]
pub fn generate_arukone(n: usize) -> PyResult<Option<PyGrid>> {
    let Ok(mut driver) = Command::new("geckodriver")
        .args(["-p", "9515"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        println!("Failed to start driver");
        return Ok(None);
    };
    let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    else {
        return Ok(None);
    };
    let result = rt.block_on(async { async_generate_arukone(n).await });
    if driver.kill().is_err() {
        println!("Failed to kill driver");
    }
    result
}

async fn async_generate_arukone(n: usize) -> PyResult<Option<PyGrid>> {
    let n = n.max(4).min(30);
    let start_time = Instant::now();
    let (grid_sender, grid_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let alive = Arc::new(AtomicBool::new(true));
    let handle = {
        let alive = alive.clone();
        tokio::spawn(async move {
            let result = start_checker(grid_receiver, output_sender, alive.clone()).await;
            alive.store(false, Ordering::Relaxed);
            result
        })
    };
    let tokio_handle = Handle::current();
    let thread = {
        let alive = alive.clone();
        let grid_sender = grid_sender.clone();
        thread::spawn(move || {
            tokio_handle.spawn(async move {
                let mut grid = Grid::new_random(n);
                while alive.load(Ordering::Relaxed) {
                    while alive.load(Ordering::Relaxed) && !grid.try_solve() {
                        grid = Grid::new_random(n);
                    }
                    if grid_sender.send(Some(grid)).is_err() {
                        break;
                    }
                    grid = Grid::new_random(n);
                }
            });
        })
    };
    let mut output = None;
    while alive.load(Ordering::Relaxed) {
        if let Ok(output_grid) = output_receiver.try_recv() {
            output = Some(output_grid);
            _ = grid_sender.send(None);
            break;
        }
        if start_time.elapsed() >= Duration::from_secs(30) {
            handle.abort();
            break;
        }
        spin_loop();
    }
    alive.store(false, Ordering::Relaxed);
    if thread.join().is_err() {
        return Ok(None);
    }
    if handle.await.is_err() {
        return Ok(None);
    }
    Ok(output.map(convert_grid))
}

fn convert_grid(grid: Grid) -> PyGrid {
    PyGrid {
        num_count: grid.num_count,
        cells: grid
            .cells
            .into_iter()
            .map(|it| {
                it.into_iter()
                    .map(|cell: generator::Cell| match cell.typ {
                        CellType::Number(num) => num,
                        _ => 0,
                    })
                    .collect()
            })
            .collect(),
    }
}
