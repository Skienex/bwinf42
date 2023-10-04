use std::fs;

use a_star::Maze;

fn main() {
    for i in 0..6 {
        let Ok(source) = fs::read_to_string(format!("zauberschule{i}.txt")) else {
            println!("Maze no. {i} could not be read");
            continue;
        };
        let mut maze = Maze::parse(&source).expect("Parse maze");
        println!("Maze no. {i}:");
        let Some(path) = maze.a_star() else {
            println!("No path was found");
            continue;
        };
        maze.dump(&path);
        println!("------------------------------")
    }
}
