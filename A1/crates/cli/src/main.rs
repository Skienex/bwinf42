use checker::{
    start_checker,
    tokio::{self, runtime::Handle, spawn},
};
use generator::Grid;
use std::{
    env::args,
    hint::spin_loop,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
};

#[tokio::main]
async fn main() {
    let mut args = args().skip(1);
    let n = args.next().unwrap().parse().unwrap();
    let threads: usize = args.next().unwrap().parse().unwrap();
    let (grid_sender, grid_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let handle = spawn(async {
        start_checker(grid_receiver, output_sender).await.unwrap();
    });
    let alive = Arc::new(AtomicBool::new(true));
    let threads: Vec<_> = (0..threads)
        .map(|_| {
            let handle = Handle::current();
            let alive = alive.clone();
            let grid_sender = grid_sender.clone();
            thread::spawn(move || {
                handle.spawn(async move {
                    let mut tries = 1;
                    let mut grid = Grid::new_random(n);
                    while alive.load(Ordering::Relaxed) {
                        while alive.load(Ordering::Relaxed) && !grid.try_solve() {
                            grid = Grid::new_random(n);
                            tries += 1;
                            if tries % 10000 == 0 {
                                println!("Tries: {}", tries);
                            }
                        }
                        if let Err(err) = grid_sender.send(Some(grid)) {
                            println!("Error sending grid: {err}");
                        }
                        grid = Grid::new_random(n);
                    }
                });
            })
        })
        .collect();
    loop {
        if let Ok(output) = output_receiver.try_recv() {
            output.dump();
            _ = grid_sender.send(None);
            break;
        }
        spin_loop();
    }
    alive.store(false, Ordering::Relaxed);
    for thread in threads {
        thread.join().expect("Thread panicked");
    }
    handle.await.unwrap();
}
