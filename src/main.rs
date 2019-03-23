#![feature(vec_remove_item, test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn worlds_hardest_test() {
        let sudoku: [u8; 81] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0,
            0, 5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
            0, 0, 0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 9,
        ];
        let mut s = SolverManager::new(sudoku);
        while { !s.next() } {}
        let mut v: Vec<u8> = Vec::new();
        for i in s.solution.iter() {
            v.push(*i);
        }
        let solution: Vec<u8> = vec![
            9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 4, 6, 1, 7, 3, 9, 8, 5, 3, 5, 1, 9, 2, 8, 7, 4, 6, 1, 2,
            8, 5, 3, 7, 6, 9, 4, 6, 3, 4, 8, 9, 2, 1, 5, 7, 7, 9, 5, 4, 6, 1, 8, 3, 2, 5, 1, 9, 2,
            8, 6, 4, 7, 3, 4, 7, 2, 3, 1, 9, 5, 6, 8, 8, 6, 3, 7, 4, 5, 2, 1, 9,
        ];
        assert_eq!(v, solution);
    }

    #[test]
    fn hardbrute_test() {
        let sudoku: [u8; 81] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0,
            0, 5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
            0, 0, 0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 9,
        ];
        let mut s = SolverManager::new(sudoku);
        while { !s.next() } {}
        let mut v: Vec<u8> = Vec::new();
        for i in s.solution.iter() {
            v.push(*i);
        }
        let solution: Vec<u8> = vec![
            9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 4, 6, 1, 7, 3, 9, 8, 5, 3, 5, 1, 9, 2, 8, 7, 4, 6, 1, 2,
            8, 5, 3, 7, 6, 9, 4, 6, 3, 4, 8, 9, 2, 1, 5, 7, 7, 9, 5, 4, 6, 1, 8, 3, 2, 5, 1, 9, 2,
            8, 6, 4, 7, 3, 4, 7, 2, 3, 1, 9, 5, 6, 8, 8, 6, 3, 7, 4, 5, 2, 1, 9,
        ];
        assert_eq!(v, solution);
    }
    #[test]
    fn random17_test() {
        let sudoku: [u8; 81] = [
            0, 0, 0, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 2, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 8, 0, 0, 0, 0,
            8, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0,
        ];
        let mut s = SolverManager::new(sudoku);
        while { !s.next() } {}
        let mut v: Vec<u8> = Vec::new();
        for i in s.solution.iter() {
            v.push(*i);
        }
        let solution: Vec<u8> = vec![
            2, 6, 4, 7, 1, 5, 8, 3, 9, 1, 3, 7, 8, 9, 2, 6, 4, 5, 5, 9, 8, 4, 3, 6, 2, 7, 1, 4, 2,
            3, 1, 7, 8, 5, 9, 6, 8, 1, 6, 5, 4, 9, 7, 2, 3, 7, 5, 9, 6, 2, 3, 4, 1, 8, 3, 7, 5, 2,
            8, 1, 9, 6, 4, 9, 8, 2, 3, 6, 4, 1, 5, 7, 6, 4, 1, 9, 5, 7, 3, 8, 2,
        ];
        assert_eq!(v, solution);
    }

    #[bench]
    fn worlds_hardest_bench(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 7, 0, 0, 9, 0, 2, 0, 0, 0, 5,
            0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 4, 5, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0,
            0, 0, 0, 6, 8, 0, 0, 8, 5, 0, 0, 0, 1, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0,
        ]);
        b.iter(|| {
            let mut s = SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
    #[bench]
    fn hardbrute_bench(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0,
            0, 5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
            0, 0, 0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 9,
        ]);
        b.iter(|| {
            let mut s = SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
    #[bench]
    fn bench_8802hard(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([
            0, 5, 0, 4, 0, 0, 9, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 5, 9, 0, 0, 0, 0, 7, 6, 3, 0, 0, 7,
            5, 0, 0, 0, 0, 0, 4, 4, 1, 0, 0, 0, 0, 7, 9, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 9,
            0, 0, 2, 7, 1, 7, 0, 0, 0, 0, 5, 4, 0, 6, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        ]);
        b.iter(|| {
            let mut s = SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
    #[bench]
    fn bench_empty(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([0; 81]);
        b.iter(|| {
            let mut s = SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
    #[bench]
    fn bench_random17(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([
            0, 0, 0, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 2, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 8, 0, 0, 0, 0,
            8, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0,
        ]);
        b.iter(|| {
            let mut s = SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
}

#[derive(Clone)]
struct Solver {
    data: [u8; 81],
    to_explore: Vec<usize>,
    options: [[bool; 10]; 81],
}

impl Solver {
    pub fn new(sudoku: [u8; 81]) -> Solver {
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

    let mut counter = 0;
    while { !s.next() } {
        counter += 1;
    }
    let mut v: Vec<u8> = Vec::new();
    for i in s.solution.iter() {
        v.push(*i);
    }
    println!("Solution:{:?}", v);
    println!("Count:{:?}", counter);
}
