# Sudoku Solver

A sudoku solver written in rust the implements naked singles and hidden singles with backtracking to achieve reasonable performance.

# How to use
sudoku_solver INPUT OUTPUT

(use sudoku_solver.exe instead if on windows)

This will read the input file which must be some number of lines, each 81 characters containing 1-9 for filled in characters, and either a space, question mark, dot or zero for unsolved squares.

It store the results in a file with them solved in the same order they were entered in the input file.

# How to compile

Requires cargo to be installed

`git clone https://github.com/RaspPiTor/sudoku_solver
cd sudoku_solver
cargo build --release
`
Compiled binary will either be at target/release/sudoku_solver for linux, or target/release/sudoku_solver.exe for windows
