mod board;

use std::io;
use crate::board::{
    Board,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    board_size: Option<usize>,
    win_threshold: Option<usize>,
}

const DIMENSIONS: usize = 2;

fn main() {
    let args = Args::parse();
    let board_size: usize;
    let win_threshold: usize;
    if let Some(s) = args.board_size {
        board_size = s;
    } else {
        board_size = 3;
    }

    if let Some(a) = args.win_threshold {
        win_threshold = a;
    } else {
        win_threshold = board_size;
    }

    let mut board = Board::new(board_size);
    start_game(&mut board, win_threshold);
}

fn start_game(board: &mut Board, win_threshold: usize) {
    'game_loop: loop {

        println!("{board}");

        let mut answer = String::new();

        io::stdin()
        .read_line(&mut answer)
        .expect("Error reading user input.");

        let answer = answer.split_whitespace();
        let answer: Vec<Result<usize, std::num::ParseIntError>> = answer.map(
            |number_str| number_str.parse()
        ).collect();

        let mut coords: Vec<usize> = vec![0; DIMENSIONS];

        for i in 0..coords.len() {
            coords[i] = match answer[i] {
                Ok(coord) => coord - 1,
                Err(_) => {
                    println!("Invalid input. Must be numeric.");
                    continue 'game_loop;
                }
            }
        }

        if coords.len() != DIMENSIONS {
            println!("Invalid input. Must enter two numbers.");
            continue 'game_loop;
        }

        let coords = (coords[0], coords[1]);

        match board.place_piece(coords) {
            Ok(_) => println!("Placed piece at {}, {}", coords.0, coords.1),
            Err(e) => {
                println!("Cannot place a piece at ({}, {})", e.0 +1, e.1 + 1);
                continue 'game_loop;
            }
        }

        match board.check_status(win_threshold) {
            board::BoardStatus::Win(player) => {
                println!("{board}");
                println!("{player} won!");
                break 'game_loop;
            },
            board::BoardStatus::Draw => {
                println!("The game ended in a draw");
                break 'game_loop;
            },
            board::BoardStatus::None => (),
        }
    }
}

