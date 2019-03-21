#![feature(vec_remove_item)]
use std::collections::HashSet;

struct Solver {
    routes: Vec<[u8; 81]>,
    rows: Vec<HashSet<u8>>,
    columns: Vec<HashSet<u8>>,
    boxes: Vec<HashSet<u8>>,
    options: HashSet<u8>,
    solution: [u8; 81],

}

impl Solver {
    pub fn new(sudoku: [u8; 81]) -> Solver {
        let mut options = HashSet::new();
        for i in 1..10 {
            options.insert(i);
        }
        let mut rows: Vec<HashSet<u8>> = Vec::new();
        for _ in 0..9 {
            rows.push(options.clone());
        }
        let mut columns: Vec<HashSet<u8>> = Vec::new();
        for _ in 0..9 {
            columns.push(options.clone());
        }
        let mut boxes: Vec<HashSet<u8>> = Vec::new();
        for _ in 0..27 {
            boxes.push(options.clone());
        }
        Solver {
            routes: vec![sudoku],
            rows: rows,
            columns: columns,
            boxes: boxes,
            options: options,
            solution: [0; 81]
        }
    }
    fn generate(&mut self, square: usize, route: &[u8; 81]) {
        let row_pos: usize = square / 9;
        let column_pos: usize = square % 9;
        let box_pos: usize = square / 27 * 3 + square / 3 % 3;
        let mut row: HashSet<u8> = HashSet::with_capacity(9);
        for i in 0..9 {
            row.insert(route[row_pos * 9 + i]);
        }
        let mut column: HashSet<u8> = HashSet::with_capacity(9);
        for i in &[0, 9, 18, 27, 36, 45, 54, 63, 72] {
            column.insert(route[column_pos + *i]);
        }
        let mut cbox: HashSet<u8> = HashSet::with_capacity(9);
        for i in &[0, 1, 2, 9, 10, 11, 18, 19, 20] {
            cbox.insert(route[box_pos / 3 * 27 + box_pos % 3 * 3 + *i]);
        }
        self.rows[row_pos] = row;
        self.columns[column_pos] = column;
        self.boxes[box_pos] = cbox;
    }
    fn process(&mut self, to_explore: &mut Vec<usize>, route: &mut [u8; 81]) -> bool {
        for i in 0..81 {
            self.generate(i, &route);
        }
        for _ in 0..to_explore.len() {
            let mut min_length = 11;
            let mut min_pos = 0;
            let mut min_result: Vec<u8> = Vec::new();
            for i in to_explore.iter() {
                let mut excluded = self.rows[i / 9].clone();
                excluded.extend(self.columns[i % 9].clone());
                excluded.extend(self.boxes[i / 27 * 3 + i / 3 % 3].clone());
                let result: Vec<&u8> = self.options.difference(&excluded).collect();
                if { result.len() < min_length } {
                    match result.len() {
                        0 => return false,
                        _ => {
                            min_length = result.len();
                            min_pos = *i;
                            min_result.clear();
                            for x in result {
                                min_result.push(*x);
                            }
                        }
                    };
                };
            }
            to_explore.remove_item(&min_pos);
            let item = min_result.pop().unwrap();
            for value in min_result.iter() {
                route[min_pos] = *value;
                self.routes.push(route.clone())
            }
            route[min_pos] = item;
            self.generate(min_pos, &route);
        }

        true
    }
    pub fn next(&mut self) -> bool {
        if { self.routes.len() > 0 } {
            let mut route = self.routes.pop().unwrap();
            let mut to_explore: Vec<usize> = Vec::new();
            for i in 0..81 {
                if { route[i] == 0 } {
                    to_explore.push(i);
                }
            }
            //println!("{:?}", to_explore);
            let result = self.process(&mut to_explore, &mut route);
            if { result } {
                self.routes.clear();
                self.solution = route;
            } else {
                return false;
            };
        }
        true
    }
}

fn main() {
    let sudoku: [u8; 81] = [
        0, 5, 0, 4, 0, 0, 9, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 5, 9, 0, 0, 0, 0, 7, 6, 3, 0, 0, 7, 5,
        0, 0, 0, 0, 0, 4, 4, 1, 0, 0, 0, 0, 7, 9, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 9, 0, 0,
        2, 7, 1, 7, 0, 0, 0, 0, 5, 4, 0, 6, 0, 0, 2, 0, 0, 0, 0, 0, 0,
    ];
    let mut s = Solver::new(sudoku);
    while { !s.next() } {
    }
    let mut v: Vec<u8> = Vec::new();
    for i in s.solution.iter() {
        v.push(*i);
    }
    println!("Solution:{:?}", v);
}
