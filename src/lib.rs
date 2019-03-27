#![feature(copy_within)]

#[derive(Clone)]
struct SudokuEmpty {
    data: [u8; 81],
    end: usize,
}
impl SudokuEmpty {
    fn new() -> SudokuEmpty {
        let mut data: [u8; 81] = [0; 81];
        for i in 0..81 {
            data[i] = i as u8;
        }
        SudokuEmpty { data, end: 81 }
    }
    #[inline(never)]
    fn remove(&mut self, square: u8) {
        if let Ok(pos) = self.data[..self.end].binary_search(&square) {
            self.data.copy_within(pos + 1..self.end, pos);
            self.end -= 1;
        }
    }
}
const SUDOKU_VALUES: [u16; 9] = [1, 2, 4, 8, 16, 32, 64, 128, 256];
const SUDOKU_MAX: u16 = 511;
const SUDOKU_CACHE: [u8; SUDOKU_MAX as usize + 1] = [
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, 5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9,
];

#[derive(Clone)]
struct Solver {
    to_explore: SudokuEmpty,
    options: [u16; 81],
}

impl Solver {
    fn new(sudoku: [u8; 81]) -> Solver {
        let to_explore: SudokuEmpty = SudokuEmpty::new();
        let mut solver = Solver {
            to_explore,
            options: [SUDOKU_MAX; 81],
        };
        for (i, item) in sudoku.iter().enumerate() {
            if *item != 0 {
                solver.generate(i, *item as usize);
            }
        }
        solver
    }
    fn generate(&mut self, square: usize, value: usize) {
        let processed_value = SUDOKU_VALUES[value - 1];
        let row_start = square / 9 * 9;
        let column_start = square % 9;
        let box_start = square / 27 * 27 + square / 3 % 3 * 3;
        let box_array: [usize; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];
        for i in 0..9 {
            let row_pos = i + row_start;
            let column_pos = column_start + 9 * i;
            let box_pos = box_start + box_array[i];
            self.options[row_pos] &= SUDOKU_MAX - processed_value;
            self.options[column_pos] &= SUDOKU_MAX - processed_value;
            self.options[box_pos] &= SUDOKU_MAX - processed_value;
        }
        self.options[square] = processed_value;
        self.to_explore.remove(square as u8);
    }
    #[inline(never)]
    fn process(&mut self, routes: &mut Vec<Solver>) -> bool {
        let mut values: Vec<u8> = Vec::with_capacity(9);
        loop {
            let mut min_length = 20;
            let mut min_pos = 0;
            let mut x: usize = 0;
            while x < self.to_explore.end {
                let pos = self.to_explore.data[x] as usize;
                let option = self.options[pos];
                let length = SUDOKU_CACHE[option as usize];
                if length < min_length {
                    match length {
                        0 => return false,
                        1 => {
                            for i in 0..9 {
                                if option & SUDOKU_VALUES[i] > 0 {
                                    self.generate(pos, i + 1);
                                    break;
                                }
                            }
                            x = 0;
                        }
                        _ => {
                            min_length = length;
                            min_pos = pos;
                            x += 1;
                        }
                    };
                } else {
                    x += 1;
                };
            }
            if min_length != 20 {
                values.clear();
                let options = self.options[min_pos];
                for i in 0..9 {
                    if options & SUDOKU_VALUES[i] > 0 {
                        values.push(i as u8 + 1);
                    }
                }
                if values.is_empty() {
                    return false;
                }
                let item = values.pop().unwrap();
                for value in values.iter() {
                    let mut clone = self.clone();
                    clone.generate(min_pos, *value as usize);
                    routes.push(clone);
                }
                self.generate(min_pos, item as usize);
            } else {
                return true;
            }
        }
    }
    fn get_result(&self) -> [u8; 81] {
        let mut solution: [u8; 81] = [0; 81];
        for (i, option) in self.options.iter().enumerate() {
            for x in 0..9 {
                if option & SUDOKU_VALUES[x] > 0 {
                    solution[i] = x as u8 + 1;
                    break;
                }
            }
        }
        solution
    }
}
#[inline(never)]
pub fn solve(sudoku: [u8; 81]) -> [u8; 81] {
    let mut routes: Vec<Solver> = vec![Solver::new(sudoku)];
    routes.reserve(32);
    while !routes.is_empty() {
        let mut route = routes.pop().unwrap();
        let result = route.process(&mut routes);
        if result {
            return route.get_result();
        }
    }
    panic!("Empty routes, but still unsolved");
}
