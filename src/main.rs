extern crate clap;
use clap::{App, Arg};
use std::io::BufRead;

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
        .get_matches();
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());
    let file =
        std::fs::File::open(matches.value_of("INPUT").unwrap()).expect("Failed to open file");
    let lines = std::io::BufReader::new(file).lines();
    let mut sudokus: Vec<[u8; 81]> = Vec::new();
    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                println!("Encountered error");
                break;
            }
        };
        if line.len() != 81 {
            println!("Wrong line length: {:?}", line);
            continue;
        }
        let mut sudoku: [u8; 81] = [0; 81];
        for (i, square) in sudoku.iter_mut().enumerate() {
            let value = match line.chars().nth(i) {
                Some(v) => match v {
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
                        break;
                    }
                },
                None => {
                    println!("Error");
                    break;
                }
            };
            *square = value;
        }
        sudokus.push(sudoku);
    }
    println!("Processed file");
    println!("Found {:?} valid sudokus", sudokus.len());
    let mut solutions: Vec<[u8; 81]> = Vec::with_capacity(sudokus.len());
    for sudoku in &sudokus {
        solutions.push(sudoku_solver::solve(*sudoku));
    }
}
