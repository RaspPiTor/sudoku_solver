extern crate clap;
extern crate msolve;
use clap::{App, Arg};
use std::io::BufRead;
use std::io::Write;

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
    let mut sudoku_count: u32 = 0;
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if line.len() != 82 && line.len() != 81 {
            println!("Wrong line length: {:?}", line);
            line.clear();
            continue;
        }
        if let Some(out_sudoku) = msolve::SudokuStruct::from(&line).solve() {
            output
                .write_all(
                    &out_sudoku
                        .to_array()
                        .iter()
                        .map(|x| x.to_string().as_bytes()[0])
                        .collect::<Vec<u8>>(),
                )
                .unwrap();
            output.write_all(b"\n").unwrap();
        }
        sudoku_count += 1;
        line.clear();
    }
    println!("Solved {:?} sudokus", sudoku_count);
}
