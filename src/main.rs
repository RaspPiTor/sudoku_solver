#![feature(vec_remove_item)]

#[derive(Clone)]
struct Solver {
    rows: [[bool; 10]; 9],
    columns: [[bool; 10]; 9],
    boxes: [[bool; 10]; 27],
    data: [u8; 81],
    to_explore: Vec<usize>,
}

impl Solver {
    pub fn new(sudoku: [u8; 81]) -> Solver {
        let rows: [[bool; 10]; 9] = [[true; 10]; 9];
        let columns: [[bool; 10]; 9] = [[true; 10]; 9];
        let boxes: [[bool; 10]; 27] = [[true; 10]; 27];
        let mut to_explore: Vec<usize> = Vec::new();
        for (i, item) in sudoku.iter().enumerate() {
            if *item == 0 {
                to_explore.push(i);
            }
        }
        let mut solver = Solver {
            rows,
            columns,
            boxes,
            data: sudoku,
            to_explore,
        };
        for i in 0..81 {
            solver.generate(i);
        }
        solver
    }
    fn generate(&mut self, square: usize) {
        let row_pos: usize = square / 9;
        let column_pos: usize = square % 9;
        let box_pos: usize = square / 27 * 3 + square / 3 % 3;
        let mut row: [bool; 10] = [true; 10];
        for i in 0..9 {
            row[self.data[row_pos * 9 + i] as usize] = false;
        }
        let mut column: [bool; 10] = [true; 10];
        for i in &[0, 9, 18, 27, 36, 45, 54, 63, 72] {
            column[self.data[column_pos + *i] as usize] = false;
        }
        let mut cbox: [bool; 10] = [true; 10];
        for i in &[0, 1, 2, 9, 10, 11, 18, 19, 20] {
            cbox[self.data[box_pos / 3 * 27 + box_pos % 3 * 3 + *i] as usize] = false;
        }
        self.rows[row_pos] = row;
        self.columns[column_pos] = column;
        self.boxes[box_pos] = cbox;
    }
    #[inline(never)]
    fn process(&mut self, routes: &mut Vec<Solver>) -> bool {
        let mut result: Vec<u8> = Vec::with_capacity(9);
        for _ in 0..self.to_explore.len() {
            let mut min_length = 11;
            let mut min_pos = 0;
            let mut min_result: Vec<u8> = Vec::new();
            for i in self.to_explore.iter() {
                result.clear();
                let row = &self.rows[i / 9];
                let column = &self.columns[i % 9];
                let cbox = &self.boxes[i / 27 * 3 + i / 3 % 3];
                for x in 1..10 {
                    if row[x] && column[x] && cbox[x] {
                        result.push(x as u8);
                        if { result.len() >= min_length } {
                            break;
                        }
                    }
                }
                if result.len() < min_length {
                    match result.len() {
                        0 => return false,
                        1 => {
                            min_pos = *i;
                            min_result = result.clone();
                            break;
                        }
                        _ => {
                            min_length = result.len();
                            min_pos = *i;
                            min_result = result.clone();
                        }
                    };
                };
            }
            self.to_explore.remove_item(&min_pos);
            let item = min_result.pop().unwrap();
            for value in min_result.iter() {
                self.data[min_pos] = *value;
                self.generate(min_pos);
                routes.push(self.clone());
            }
            self.data[min_pos] = item;
            self.generate(min_pos);
        }

        true
    }
}

struct SolverManager {
    routes: Vec<Solver>,
    solution: [u8; 81],
}

impl SolverManager {
    pub fn new(sudoku: [u8; 81]) -> SolverManager {
        SolverManager {
            routes: vec![Solver::new(sudoku)],
            solution: [0; 81],
        }
    }
    #[inline(never)]
    pub fn next(&mut self) -> bool {
        if !self.routes.is_empty() {
            let mut route = self.routes.pop().unwrap();
            let result = route.process(&mut self.routes);
            if result {
                self.solution = route.data;
                self.routes.clear();
            } else {
                return false;
            };
        }
        true
    }
}

fn main() {
    let sudoku: [u8; 81] = [
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 7, 0, 0, 9, 0, 2, 0, 0, 0, 5, 0,
        0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 4, 5, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0,
        0, 6, 8, 0, 0, 8, 5, 0, 0, 0, 1, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0,
    ];
    let mut s = SolverManager::new(sudoku);
    while { !s.next() } {}
    let mut v: Vec<u8> = Vec::new();
    for i in s.solution.iter() {
        v.push(*i);
    }
    println!("Solution:{:?}", v);
}
