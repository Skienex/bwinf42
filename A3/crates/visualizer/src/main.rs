use std::{fs, path::PathBuf};

use a_star::{CellType, Maze};
use raylib::{
    prelude::{Color, MouseButton, RaylibDraw, RaylibDrawHandle, Vector2},
    text::Font,
    RaylibHandle, RaylibThread,
};

// Colors
const BACKGROUND: Color = Color::BLACK;
const TITLE_FOREGROUND: Color = Color::GREEN;
const HEADER_FOREGROUND: Color = Color::new(200, 200, 200, 255);
const LIST_FOREGROUND: Color = Color::WHITE;
const EMPTY: Color = Color::new(50, 50, 50, 255);
const WALL: Color = Color::new(20, 20, 20, 255);
const START: Color = Color::GREEN;
const GOAL: Color = Color::GOLD;
const PATH: Color = Color::new(175, 175, 175, 255);

// Constants
const TITLE_TOP_OFFSET: i32 = 10;
const HEADER_TOP_OFFSET: i32 = 70;
const LINE_TOP_OFFSET: i32 = 100;
const LIST_TOP_OFFSET: i32 = 125;
const LIST_ENTRY_HEIGHT: i32 = 25;
const LIST_FONT_SIZE: i32 = 23;

fn main() {
    let (mut rl, thread) = raylib::init().size(50, 50).title("A* maze-solving").build();
    rl.set_target_fps(60);
    let font = rl.load_font(&thread, "DGi22TI4tVon9GmlfGUh3uY8.ttf.fnt");
    let font = match font {
        Ok(font) => {
            rl.gui_set_font(&font);
            Some(font)
        }
        Err(err) => {
            eprintln!("Could not load font: {err}");
            None
        }
    };
    start_menu(&mut rl, &thread, font.as_ref());
}

fn start_menu(rl: &mut RaylibHandle, thread: &RaylibThread, font: Option<&Font>) {
    let mut files = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let Ok(entry) = entry else {
            continue;
        };
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if !name.ends_with(".txt") {
            continue;
        }
        files.push((name.to_string(), entry.path()));
    }
    files.sort_by(|(l_name, _), (r_name, _)| l_name.cmp(r_name));
    let window_width = 900;
    let window_height = 506;
    rl.set_window_size(window_width, window_height);
    rl.set_window_position(1920 / 2 - window_width / 2, 1080 / 2 - window_height / 2);
    while !rl.window_should_close() {
        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            let Vector2 { x: _, y } = rl.get_mouse_position();
            let y = y as usize;
            let mut path = None;
            for (i, (_, file_path)) in files.iter().enumerate() {
                let top_height = LIST_TOP_OFFSET as usize + i * LIST_ENTRY_HEIGHT as usize;
                let bottom_height = top_height + 15;
                if (top_height..=bottom_height).contains(&y) {
                    path = Some(file_path);
                    break;
                }
            }
            if let Some(path) = path {
                let pos = rl.get_window_position();
                start_maze(rl, thread, path);
                rl.set_window_size(window_width, window_height);
                rl.set_window_position(pos.x as i32, pos.y as i32);
                continue;
            }
        }
        let mut d = rl.begin_drawing(thread);
        d.clear_background(BACKGROUND);
        draw_menu(&mut d, &files, window_width, font);
    }
}

fn draw_menu(
    d: &mut RaylibDrawHandle,
    files: &[(String, PathBuf)],
    width: i32,
    font: Option<&Font>,
) {
    draw_text(
        d,
        font,
        "Maze Solver",
        40,
        TITLE_TOP_OFFSET,
        60,
        TITLE_FOREGROUND,
    );
    draw_text(
        d,
        font,
        "BwInf 42",
        40,
        HEADER_TOP_OFFSET,
        20,
        HEADER_FOREGROUND,
    );
    d.draw_line(
        40,
        LINE_TOP_OFFSET,
        width,
        LINE_TOP_OFFSET,
        HEADER_FOREGROUND,
    );
    for (i, (name, _)) in files.iter().enumerate() {
        draw_text(
            d,
            font,
            name,
            40,
            LIST_TOP_OFFSET + i as i32 * LIST_ENTRY_HEIGHT,
            LIST_FONT_SIZE,
            LIST_FOREGROUND,
        );
    }
}

fn start_maze(rl: &mut RaylibHandle, thread: &RaylibThread, path: &PathBuf) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("{path:?} could not be read");
        return false;
    };
    let Some(mut maze) = Maze::parse(&source) else {
        eprintln!("{path:?} does not contain a valid maze");
        return false;
    };
    eprintln!("{path:?} was loaded");
    let maze_path = maze.a_star().unwrap_or_else(|| {
        eprintln!("no path was found for {path:?}");
        vec![]
    });
    if !maze_path.is_empty() {
        eprintln!("{path:?} was solved");
    }
    let [dx, dy, dz] = maze.dimensions;
    let max_window_width = 1800;
    let max_window_height = 1012;
    let cell_size = (max_window_width / (dx * dz))
        .min(max_window_height / dy)
        .min(25);
    let window_width = cell_size * dx * dz + cell_size + 2;
    let window_height = cell_size * dy + 2;
    rl.set_window_size(window_width as i32, window_height as i32);
    rl.set_window_position(
        1920 / 2 - window_width as i32 / 2,
        1080 / 2 - window_height as i32 / 2,
    );
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(BACKGROUND);
        draw_maze(&mut d, &maze, cell_size, &maze_path);
    }
    true
}

fn draw_maze(d: &mut RaylibDrawHandle, maze: &Maze, cell_size: usize, path: &[[i16; 3]]) {
    let [dx, ..] = maze.dimensions;
    for (z, layer) in maze.cells.iter().enumerate() {
        for (y, row) in layer.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                d.draw_rectangle(
                    (1 + cell_size * x + cell_size * (dx + 1) * z) as i32,
                    (1 + cell_size * y) as i32,
                    cell_size as i32,
                    cell_size as i32,
                    match cell.typ {
                        CellType::Empty => {
                            if path.contains(&[x as i16, y as i16, z as i16]) {
                                PATH
                            } else {
                                EMPTY
                            }
                        }
                        CellType::Wall => WALL,
                        CellType::Start => START,
                        CellType::Goal => GOAL,
                    },
                );
            }
        }
    }
}

fn draw_text(
    d: &mut RaylibDrawHandle,
    font: Option<&Font>,
    text: &str,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
) {
    if let Some(font) = font {
        d.draw_text_ex(
            font,
            text,
            Vector2::new(x as f32, y as f32),
            font_size as f32,
            2.0,
            color,
        );
        return;
    }
    d.draw_text(text, x, y, font_size, color);
}
