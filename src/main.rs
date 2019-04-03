extern crate clap;
use clap::{App, Arg};
use std::io::BufRead;
use std::io::Write;

use sudoku_solver;

fn main() {
    let matches = App::new("Sudoku Solver")
        .version("0.1.0")
        .arg(
            Arg::with_name("INPUT")
                .help("File containig sudoku's")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("File containig sudoku's")
                .required(true)
                .index(2),
        )
        .get_matches();
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());
    let file_in =
        std::fs::File::open(matches.value_of("INPUT").unwrap()).expect("Failed to open file");
    let file_out =
        std::fs::File::create(matches.value_of("OUTPUT").unwrap()).expect("Failed to open file");
    let mut buf = std::io::BufReader::new(file_in);
    let mut output = std::io::BufWriter::new(file_out);
    let mut result: [u8; 82] = [b'\n'; 82];
    let mut sudoku_count: u32 = 0;
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if line.len() != 82 && line.len() != 81 {
            println!("Wrong line length: {:?}", line);
            line.clear();
            continue;
        }
        let mut sudoku: [u8; 81] = [0; 81];
        for (square, char) in sudoku.iter_mut().zip(line.chars()) {
            *square = match char {
                ' ' => 0,
                '?' => 0,
                '.' => 0,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                v => {
                    println!("Invalid char:{:?} in line: {:?}", v, line);
                    line.clear();
                    break;
                }
            };
        }
        let out_sudoku = sudoku_solver::solve(sudoku);
        for (r, s) in result.iter_mut().zip(out_sudoku.iter()) {
            *r = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'][*s as usize];
        }
        match output.write(&result) {
            Ok(_) => {}
            Err(_) => println!("Encountered error when writing"),
        };
        sudoku_count += 1;
        line.clear();
    }
    println!("Solved {:?} sudokus", sudoku_count);
}
