use checker::{
    start_checker,
    tokio::{self, spawn},
};
use generator::Grid;
use std::{env::args, sync::mpsc};

#[tokio::main]
async fn main() {
    let n = args().nth(1).unwrap().parse().unwrap();
    let (grid_sender, grid_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let handle = spawn(async {
        start_checker(grid_receiver, output_sender).await.unwrap();
    });
    let mut tries = 1;
    let mut grid = Grid::new_random(n);
    loop {
        while !grid.try_solve() {
            grid = Grid::new_random(n);
            tries += 1;
            if tries % 1000 == 0 {
                println!("Tries: {}", tries);
            }
        }
        if let Ok(output) = output_receiver.try_recv() {
            output.dump();
            _ = grid_sender.send(None);
            break;
        }
        if let Err(err) = grid_sender.send(Some(grid)) {
            println!("Error sending grid: {err}");
        }
        grid = Grid::new_random(n);
    }
    handle.await.unwrap();
}
