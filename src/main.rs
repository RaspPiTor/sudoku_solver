use sudoku_solver;

fn main() {
    loop {
        let sudoku: [u8; 81] = [
            0, 5, 0, 4, 0, 0, 9, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 5, 9, 0, 0, 0, 0, 7, 6, 3, 0, 0, 7,
            5, 0, 0, 0, 0, 0, 4, 4, 1, 0, 0, 0, 0, 7, 9, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 9,
            0, 0, 2, 7, 1, 7, 0, 0, 0, 0, 5, 4, 0, 6, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        ];
        let mut s = sudoku_solver::SolverManager::new(sudoku);

        let mut counter = 0;
        while { !s.next() } {
            counter += 1;
        }
        let mut v: Vec<u8> = Vec::new();
        for i in s.solution.iter() {
            v.push(*i);
        }
        //println!("Solution:{:?}", v);
        //println!("Count:{:?}", counter);
    }
}
