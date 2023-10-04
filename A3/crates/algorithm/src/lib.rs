use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display, ops::Add};

use yansi::{Color, Paint};

pub const DIRECTIONS: [Direction; 6] = [
    Direction::PlusX,
    Direction::MinusX,
    Direction::PlusY,
    Direction::MinusY,
    Direction::PlusZ,
    Direction::MinusZ,
];

pub struct Maze {
    pub dimensions: [usize; 3],
    pub cells: [Vec<Vec<Cell>>; 2],
}

impl Maze {
    pub fn parse(source: &str) -> Option<Self> {
        let mut lines = source.lines();
        let mut metadata = lines.next()?.split(' ');
        let y_width: usize = metadata.next()?.parse().ok()?;
        let x_width: usize = metadata.next()?.parse().ok()?;
        let mut maze = Self {
            dimensions: [x_width, y_width, 2],
            cells: [Vec::new(), Vec::new()],
        };
        for z in 0..2 {
            for y in 0..y_width {
                maze.cells[z].push(Vec::new());
                let line = lines.next()?;
                for x in 0..x_width {
                    let c = &line[x..x + 1];
                    let typ = match c {
                        "#" => CellType::Wall,
                        "." => CellType::Empty,
                        "A" => CellType::Start,
                        "B" => CellType::Goal,
                        _ => panic!("{c}"),
                    };
                    maze.cells[z][y].push(Cell {
                        typ,
                        g_cost: None,
                        h_cost: None,
                        parent: None,
                        open: false,
                        closed: false,
                    });
                }
            }
            lines.next();
        }
        Some(maze)
    }

    pub fn at_mut(&mut self, [x, y, z]: [i16; 3]) -> Option<&mut Cell> {
        self.cells
            .get_mut(z as usize)?
            .get_mut(y as usize)?
            .get_mut(x as usize)
    }

    pub fn at(&self, [x, y, z]: [i16; 3]) -> Option<&Cell> {
        self.cells.get(z as usize)?.get(y as usize)?.get(x as usize)
    }

    pub fn at_ref(&self, pos: [i16; 3]) -> Option<CellRef> {
        let cell = self.at(pos)?;
        Some(CellRef {
            pos,
            g_cost: cell.g_cost?,
            h_cost: cell.h_cost?,
        })
    }

    pub fn find(&self, typ: CellType) -> Option<[i16; 3]> {
        for (z, layer) in self.cells.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if cell.typ == typ {
                        return Some([x as i16, y as i16, z as i16]);
                    }
                }
            }
        }
        None
    }

    pub fn collect_path(&self, end: [i16; 3]) -> Vec<[i16; 3]> {
        let mut path = Vec::new();
        let mut last = end;
        while let Some(direction) = self.at(last).unwrap().parent {
            path.push(last);
            last = direction + last;
        }
        path.push(last);
        path
    }

    pub fn a_star(&mut self) -> Option<Vec<[i16; 3]>> {
        let start = self.find(CellType::Start)?;
        let goal = self.find(CellType::Goal)?;
        let start_cell = self.at_mut(start)?;
        start_cell.g_cost = Some(0);
        start_cell.h_cost = Some(calculate_h_cost(start, goal));
        start_cell.open = true;
        let mut open = BinaryHeap::new();
        open.push(self.at_ref(start).unwrap());
        while let Some(CellRef {
            pos: current_pos,
            g_cost: current_g_cost,
            h_cost: _,
        }) = open.pop()
        {
            let current = self.at_mut(current_pos).unwrap();
            current.open = false;
            current.closed = true;
            if current_pos == goal {
                return Some(self.collect_path(current_pos));
            }
            for direction in DIRECTIONS {
                let neighbor_pos = direction + current_pos;
                let Some(neighbor) = self.at_mut(neighbor_pos) else {
                    continue;
                };
                if !neighbor.typ.is_traversable() || neighbor.closed {
                    continue;
                }
                if neighbor.h_cost.is_none() {
                    neighbor.h_cost = Some(calculate_h_cost(neighbor_pos, goal));
                }
                let new_g_cost = current_g_cost + direction.cost();
                if new_g_cost < neighbor.g_cost.unwrap_or(u32::MAX) || !neighbor.open {
                    neighbor.g_cost = Some(new_g_cost);
                    neighbor.parent = Some(direction.invert());
                    neighbor.open = true;
                    open.push(self.at_ref(neighbor_pos).unwrap());
                }
            }
        }
        None
    }

    pub fn dump(&self, path: &[[i16; 3]]) {
        for (z, layer) in self.cells.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let mark = path.contains(&[x as i16, y as i16, z as i16]);
                    if mark {
                        print!("{}", Paint::new(cell.typ).bg(Color::Green));
                    } else {
                        print!("{}", cell.typ);
                    }
                }
                println!();
            }
            println!();
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    PlusX,
    MinusX,
    PlusY,
    MinusY,
    PlusZ,
    MinusZ,
}

impl Direction {
    pub fn cost(self) -> u32 {
        match self {
            Self::PlusX | Self::MinusX | Self::PlusY | Self::MinusY => 1,
            Self::PlusZ | Self::MinusZ => 3,
        }
    }

    pub fn invert(self) -> Self {
        match self {
            Self::PlusX => Self::MinusX,
            Self::MinusX => Self::PlusX,
            Self::PlusY => Self::MinusY,
            Self::MinusY => Self::PlusY,
            Self::PlusZ => Self::MinusZ,
            Self::MinusZ => Self::PlusZ,
        }
    }
}

impl Add<[i16; 3]> for Direction {
    type Output = [i16; 3];

    fn add(self, [x, y, z]: [i16; 3]) -> [i16; 3] {
        match self {
            Self::PlusX => [x + 1, y, z],
            Self::MinusX => [x - 1, y, z],
            Self::PlusY => [x, y + 1, z],
            Self::MinusY => [x, y - 1, z],
            Self::PlusZ => [x, y, z + 1],
            Self::MinusZ => [x, y, z - 1],
        }
    }
}

pub struct Cell {
    pub typ: CellType,
    pub g_cost: Option<u32>,
    pub h_cost: Option<u32>,
    pub parent: Option<Direction>,
    pub open: bool,
    pub closed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Wall,
    Start,
    Goal,
}

impl CellType {
    pub fn is_traversable(self) -> bool {
        matches!(self, Self::Empty | Self::Goal | Self::Start)
    }
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "  "),
            Self::Start => write!(f, "AA"),
            Self::Goal => write!(f, "BB"),
            Self::Wall => write!(f, "██"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellRef {
    pub pos: [i16; 3],
    pub g_cost: u32,
    pub h_cost: u32,
}

impl CellRef {
    pub fn f_cost(self) -> u32 {
        self.g_cost + self.h_cost
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for CellRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            match self.f_cost().cmp(&other.f_cost()) {
                Ordering::Equal => self.h_cost.cmp(&other.h_cost),
                ord => ord,
            }
            .reverse(),
        )
    }
}

impl Ord for CellRef {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn calculate_h_cost(from: [i16; 3], to: [i16; 3]) -> u32 {
    let [ax, ay, az] = from;
    let [bx, by, bz] = to;
    let [cx, cy, cz] = [ax - bx, ay - by, az - bz];
    (((cx * cx + cy * cy + cz * cz) as f64).sqrt() * 10.0) as u32
}
