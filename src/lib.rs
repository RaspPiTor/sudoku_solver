#![feature(vec_remove_item)]

#[derive(Clone)]
struct Solver {
    data: [u8; 81],
    to_explore: Vec<usize>,
    options: [[bool; 10]; 81],
}

impl Solver {
    fn new(sudoku: [u8; 81]) -> Solver {
        let mut to_explore: Vec<usize> = Vec::new();
        for (i, item) in sudoku.iter().enumerate() {
            if *item == 0 {
                to_explore.push(i);
            }
        }
        let mut solver = Solver {
            data: sudoku,
            to_explore,
            options: [[true; 10]; 81],
        };
        for i in 0..81 {
            if sudoku[i] != 0 {
                solver.generate(i);
            }
        }
        solver
    }
    fn generate(&mut self, square: usize) {
        let value = self.data[square] as usize;
        let mut valid = [false; 10];
        valid[value] = true;
        self.options[square] = valid;
        let row_start = square / 9 * 9;
        for i in 0..9 {
            let pos = i + row_start;
            if pos != square {
                self.options[pos][value] = false;
            }
        }
        let column_start = square % 9;
        for i in 0..9 {
            let pos = column_start + 9 * i;
            if pos != square {
                self.options[pos][value] = false;
            }
        }
        let box_start = square / 27 * 27 + square / 3 % 3 * 3;
        for i in &[0, 1, 2, 9, 10, 11, 18, 19, 20] {
            let pos = box_start + *i;
            if pos != square {
                self.options[pos][value] = false;
            }
        }
    }
    #[inline(never)]
    fn process(&mut self, routes: &mut Vec<Solver>) -> bool {
        let mut values: Vec<u8> = Vec::with_capacity(9);
        for _ in 0..self.to_explore.len() {
            let mut min_length = 20;
            let mut min_pos = 0;
            let mut min_result: [bool; 10] = [true; 10];
            for i in self.to_explore.iter() {
                let result = self.options[*i];
                let mut length: u8 = 0;
                for x in 1..10 {
                    if result[x] {
                        length += 1;
                    }
                }
                if length < min_length {
                    match length {
                        0 => return false,
                        1 => {
                            min_pos = *i;
                            min_result = result.clone();
                            break;
                        }
                        _ => {
                            min_length = length;
                            min_pos = *i;
                            min_result = result.clone();
                        }
                    };
                };
            }
            self.to_explore.remove_item(&min_pos);
            values.clear();
            for i in 1..10 {
                if min_result[i] {
                    values.push(i as u8);
                }
            }
            let item = values.pop().unwrap();
            for value in values.iter() {
                let mut clone = self.clone();
                clone.data[min_pos] = *value;
                clone.generate(min_pos);
                routes.push(clone);
            }
            self.data[min_pos] = item;
            self.generate(min_pos);
        }

        true
    }
}

pub struct SolverManager {
    routes: Vec<Solver>,
    pub solution: [u8; 81],
}

impl SolverManager {
    pub fn new(sudoku: [u8; 81]) -> SolverManager {
        SolverManager {
            routes: vec![Solver::new(sudoku)],
            solution: [0; 81],
        }
    }
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
