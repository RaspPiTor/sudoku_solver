#![feature(vec_remove_item, test)]
use sudoku_solver;
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn worlds_hardest_bench(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 7, 0, 0, 9, 0, 2, 0, 0, 0, 5,
            0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 4, 5, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0,
            0, 0, 0, 6, 8, 0, 0, 8, 5, 0, 0, 0, 1, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0,
        ]);
        b.iter(|| {
            let mut s = sudoku_solver::SolverManager::new(sudoku);
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
            let mut s = sudoku_solver::SolverManager::new(sudoku);
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
            let mut s = sudoku_solver::SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
    #[bench]
    fn bench_empty(b: &mut Bencher) {
        let sudoku: [u8; 81] = test::black_box([0; 81]);
        b.iter(|| {
            let mut s = sudoku_solver::SolverManager::new(sudoku);
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
            let mut s = sudoku_solver::SolverManager::new(sudoku);
            while { !s.next() } {}
            test::black_box(&s);
        });
    }
}
