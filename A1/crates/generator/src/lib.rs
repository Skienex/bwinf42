use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    env::args,
    fmt::{Debug, Display},
    ops::Add,
};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use yansi::{Color, Paint};

pub const DIRECTIONS: [Direction; 4] = [
    Direction::PlusX,
    Direction::MinusX,
    Direction::PlusY,
    Direction::MinusY,
];
pub const SORTERS: [PairSorter; 3] = [
    PairSorter::Descending,
    PairSorter::Shuffle,
    PairSorter::Ascending,
];

pub struct Grid {
    num_count: u16,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new_random(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = Self {
            num_count: rng.gen_range(n as u16 / 2..2 * n as u16),
            cells: Vec::new(),
        };
        for _ in 0..n {
            let mut row = Vec::new();
            for _ in 0..n {
                row.push(Cell::default());
            }
            grid.cells.push(row);
        }
        for number in 1..=grid.num_count {
            let [mut start_pos, mut end_pos] = random_pair(&mut rng, n);
            while start_pos == end_pos
                || grid.at(start_pos).unwrap().typ != CellType::Empty
                || grid.at(end_pos).unwrap().typ != CellType::Empty
            {
                [start_pos, end_pos] = random_pair(&mut rng, n);
            }
            grid.at_mut(start_pos).unwrap().typ = CellType::Number(number);
            grid.at_mut(end_pos).unwrap().typ = CellType::Number(number);
        }
        grid
    }

    pub fn at_mut(&mut self, [x, y]: [i16; 2]) -> Option<&mut Cell> {
        self.cells.get_mut(y as usize)?.get_mut(x as usize)
    }

    pub fn at(&self, [x, y]: [i16; 2]) -> Option<&Cell> {
        self.cells.get(y as usize)?.get(x as usize)
    }

    pub fn at_ref(&self, pos: [i16; 2]) -> Option<CellRef> {
        let cell = self.at(pos)?;
        Some(CellRef {
            pos,
            g_cost: cell.g_cost?,
            h_cost: cell.h_cost?,
        })
    }

    pub fn find(&self, number: u16) -> Option<[i16; 2]> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let CellType::Number(num) = cell.typ {
                    if number == num {
                        return Some([x as i16, y as i16]);
                    }
                }
            }
        }
        None
    }

    pub fn find_pair(&self, number: u16) -> Option<[[i16; 2]; 2]> {
        let mut first = None;
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let CellType::Number(num) = cell.typ {
                    if number == num {
                        let current = [x as i16, y as i16];
                        if let Some(first) = first {
                            return Some([first, current]);
                        }
                        first = Some(current);
                    }
                }
            }
        }
        None
    }

    pub fn find_pairs(&self) -> Option<Vec<[[i16; 2]; 2]>> {
        let mut pairs = Vec::new();
        for i in 1..=self.num_count {
            let Some(pair) = self.find_pair(i) else {
                return None;
            };
            pairs.push(pair);
        }
        Some(pairs)
    }

    pub fn clear(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if let CellType::Path(_) = cell.typ {
                    cell.typ = CellType::Empty;
                }
                cell.g_cost = None;
                cell.h_cost = None;
                cell.parent = None;
                cell.flags.open(false);
                cell.flags.close(false);
            }
        }
    }

    pub fn partial_clear(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.g_cost = None;
                cell.h_cost = None;
                cell.parent = None;
                cell.flags.open(false);
                cell.flags.close(false);
            }
        }
    }

    pub fn mark_path(&mut self, end_pos: [i16; 2], number: u16) {
        let mut last = end_pos;
        while let Some(direction) = self.at(last).unwrap().parent {
            // collect_path
            let cell = self.at_mut(last).unwrap();
            if let CellType::Empty = cell.typ {
                cell.typ = CellType::Path(number);
            }
            last = direction + last;
        }
    }

    pub fn a_star(&mut self, start_pos: [i16; 2], end_pos: [i16; 2]) -> Option<()> {
        self.partial_clear();
        let start_cell = self.at_mut(start_pos)?;
        start_cell.g_cost = Some(0);
        start_cell.h_cost = Some(calculate_h_cost(start_pos, end_pos));
        start_cell.flags.open(true);
        let CellType::Number(number) = start_cell.typ else {
            panic!("Invalid starting cell: {start_cell:#?}@{start_pos:?}");
        };
        let mut open = BinaryHeap::new();
        open.push(self.at_ref(start_pos)?);
        while let Some(CellRef {
            pos: current_pos,
            g_cost: current_g_cost,
            h_cost: _,
        }) = open.pop()
        {
            let current = self.at_mut(current_pos).unwrap();
            current.flags.open(false);
            current.flags.close(true);
            if current_pos == end_pos {
                self.mark_path(end_pos, number);
                return Some(());
            }
            for direction in DIRECTIONS {
                let neighbor_pos = direction + current_pos;
                let Some(neighbor) = self.at_mut(neighbor_pos) else {
                    continue;
                };
                if !neighbor.is_traversable(number) || neighbor.flags.is_closed() {
                    continue;
                }
                if neighbor.h_cost.is_none() {
                    neighbor.h_cost = Some(calculate_h_cost(neighbor_pos, end_pos));
                }
                let new_g_cost = current_g_cost + 1;
                if new_g_cost < neighbor.g_cost.unwrap_or(u32::MAX) || !neighbor.flags.is_open() {
                    neighbor.g_cost = Some(new_g_cost);
                    neighbor.parent = Some(direction.invert());
                    neighbor.flags.open(true);
                    open.push(self.at_ref(neighbor_pos).unwrap());
                }
            }
        }
        None
    }

    pub fn try_solve(&mut self) -> bool {
        let Some(mut pairs) = self.find_pairs() else {
            return false;
        };
        //if self.can_solve_with(PairSorter::Ascending, &mut pairs) {
        //    return false;
        //}
        for sorter in SORTERS {
            if self.can_solve_with(sorter, &mut pairs) {
                return true;
            }
        }
        false
    }

    pub fn can_solve_with(&mut self, sorter: PairSorter, pairs: &mut [[[i16; 2]; 2]]) -> bool {
        sorter.sort(pairs);
        self.can_solve(pairs)
    }

    /// - [x] finde Nummernpaare
    /// - [x] sortiere nach Distanz, aufsteigend
    /// - [x] probiere alle Pfade zu finden (`a_star`)
    pub fn can_solve(&mut self, pairs: &[[[i16; 2]; 2]]) -> bool {
        self.clear();
        for &[start_pos, end_pos] in pairs {
            if self.a_star(start_pos, end_pos).is_none() {
                return false;
            }
        }
        true
    }

    pub fn dump(&self) {
        println!("{}\n{}", self.cells.len(), self.num_count);
        for row in &self.cells {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{}\n{}\n", self.cells.len(), self.num_count));
        for row in &self.cells {
            for cell in row {
                output.push_str(&format!("{:?} ", cell.typ));
            }
            output.push('\n');
        }
        output
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    PlusX,
    MinusX,
    PlusY,
    MinusY,
}

impl Direction {
    pub fn invert(self) -> Self {
        match self {
            Self::PlusX => Self::MinusX,
            Self::MinusX => Self::PlusX,
            Self::PlusY => Self::MinusY,
            Self::MinusY => Self::PlusY,
        }
    }
}

impl Add<[i16; 2]> for Direction {
    type Output = [i16; 2];

    fn add(self, [x, y]: [i16; 2]) -> Self::Output {
        match self {
            Self::PlusX => [x + 1, y],
            Self::MinusX => [x - 1, y],
            Self::PlusY => [x, y + 1],
            Self::MinusY => [x, y - 1],
        }
    }
}

#[derive(Debug, Default)]
pub struct Cell {
    pub typ: CellType,
    pub g_cost: Option<u32>,
    pub h_cost: Option<u32>,
    pub parent: Option<Direction>,
    pub flags: CellFlags,
}

impl Cell {
    pub fn is_traversable(&self, number: u16) -> bool {
        match self.typ {
            CellType::Empty => true,
            CellType::Number(num) if number == num => true,
            CellType::Path(num) if number == num => true,
            _ => false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const COLORS: [Color; 6] = [
            Color::Green,
            Color::Red,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ];
        match self.typ {
            CellType::Empty => write!(f, "0"),
            CellType::Number(num) => write!(
                f,
                "{}",
                Paint::new(num).bg(COLORS[num as usize % COLORS.len()])
            ),
            CellType::Path(num) => write!(
                f,
                "{}",
                Paint::new("0").bg(COLORS[num as usize % COLORS.len()])
            ),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum CellType {
    Empty,
    Number(u16),
    Path(u16),
}

impl Debug for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "0"),
            Self::Number(num) => write!(f, "{num}"),
            Self::Path(_) => write!(f, "0"),
        }
    }
}

impl Default for CellType {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CellFlags(u8);

impl CellFlags {
    pub fn is_open(self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn open(&mut self, value: bool) {
        self.0 |= value as u8;
    }

    pub fn is_closed(self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn close(&mut self, value: bool) {
        self.0 |= (value as u8) << 1;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellRef {
    pub pos: [i16; 2],
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

#[derive(Clone, Copy)]
pub enum PairSorter {
    Ascending,
    Descending,
    Shuffle,
}

impl PairSorter {
    pub fn sort(self, pairs: &mut [[[i16; 2]; 2]]) {
        match self {
            Self::Ascending => {
                pairs.sort_by(|[[ax, ay], [bx, by]], [[cx, cy], [dx, dy]]| {
                    let dist_a = (((ax - bx).pow(2) + (ay - by).pow(2)) as f32).sqrt();
                    let dist_b = (((cx - dx).pow(2) + (cy - dy).pow(2)) as f32).sqrt();
                    dist_a.partial_cmp(&dist_b).unwrap()
                });
            }
            Self::Descending => {
                pairs.sort_by(|[[ax, ay], [bx, by]], [[cx, cy], [dx, dy]]| {
                    let dist_a = (((ax - bx).pow(2) + (ay - by).pow(2)) as f32).sqrt();
                    let dist_b = (((cx - dx).pow(2) + (cy - dy).pow(2)) as f32).sqrt();
                    dist_a.partial_cmp(&dist_b).unwrap()
                });
            }
            Self::Shuffle => pairs.shuffle(&mut thread_rng()),
        }
    }
}

pub fn calculate_h_cost(from: [i16; 2], to: [i16; 2]) -> u32 {
    let [ax, ay] = from;
    let [bx, by] = to;
    let [cx, cy] = [ax - bx, ay - by];
    (((cx * cx + cy * cy) as f64).sqrt() * 10.0) as u32
}

pub fn random_pair(rng: &mut ThreadRng, n: usize) -> [[i16; 2]; 2] {
    fn rand_range(rng: &mut ThreadRng, n: usize) -> i16 {
        rng.gen_range(0..n) as i16
    }
    [
        [rand_range(rng, n), rand_range(rng, n)],
        [rand_range(rng, n), rand_range(rng, n)],
    ]
}

fn _main() {
    let n = args().nth(1).unwrap().parse().unwrap();
    let mut tries = 1;
    let mut grid = Grid::new_random(n);
    while !grid.try_solve() {
        grid = Grid::new_random(n);
        tries += 1;
        if tries % 1000 == 0 {
            println!("Tries: {tries}");
        }
    }
    println!("Tries: {tries}");
    grid.dump();
    // grid.clear();
    // grid.dump();
}
