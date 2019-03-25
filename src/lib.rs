use std::collections::HashSet;

#[derive(Clone)]
struct Solver {
    data: [u8; 81],
    to_explore: HashSet<usize>,
    options: [[bool; 10]; 81],
}

impl Solver {
    fn new(sudoku: [u8; 81]) -> Solver {
        let mut to_explore: HashSet<usize> = HashSet::with_capacity(64);
        for (i, item) in sudoku.iter().enumerate() {
            if *item == 0 {
                to_explore.insert(i);
            }
        }
        let mut solver = Solver {
            data: sudoku,
            to_explore,
            options: [[true; 10]; 81],
        };
        for (i, item) in sudoku.iter().enumerate() {
            if *item != 0 {
                solver.generate(i);
            }
        }
        solver
    }
    fn generate(&mut self, square: usize) {
        let value = self.data[square] as usize;
        //let mut valid = [false; 10];
        //valid[value] = true;
        //self.options[square] = valid;
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
        let mut one_vec: Vec<(usize, u8)> = Vec::with_capacity(20);
        while !self.to_explore.is_empty() {
            let mut min_length = 20;
            let mut min_pos = 0;
            let mut min_result: [bool; 10] = [true; 10];
            one_vec.clear();
            for i in self.to_explore.iter() {
                let result = self.options[*i];
                let mut length: u8 = 0;
                for item in &result[1..] {
                    if *item {
                        length += 1;
                    }
                }
                if length < min_length {
                    match length {
                        0 => return false,
                        1 => {
                            for (x, item) in result.iter().enumerate().skip(1) {
                                if *item {
                                    one_vec.push((*i, x as u8));
                                    break;
                                }
                            }
                        }
                        _ => {
                            min_length = length;
                            min_pos = *i;
                            min_result = result;
                        }
                    };
                };
            }

            values.clear();
            if !one_vec.is_empty() {
                for (pos, value) in &one_vec {
                    if self.options[*pos][*value as usize] {
                        self.data[*pos] = *value;
                        self.generate(*pos);
                        self.to_explore.remove(pos);
                    } else {
                        return false;
                    }
                }
            } else {
                self.to_explore.remove(&min_pos);
                for (i, item) in min_result.iter().enumerate().skip(1) {
                    if *item {
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
        }

        true
    }
}

pub fn solve(sudoku: [u8; 81]) -> [u8; 81] {
    let mut routes: Vec<Solver> = vec![Solver::new(sudoku)];
    while !routes.is_empty() {
        let mut route = routes.pop().unwrap();
        let result = route.process(&mut routes);
        if result {
            return route.data;
        }
    }
    panic!("Empty routes, but still unsolved");
}
