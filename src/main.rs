mod board;

use std::io;
use crate::board::{
    Board,
};

const DIMENSIONS: usize = 2;

fn main() {
    let mut board = Board::new(3);
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

        if coords
            .iter()
            .any(|coord| *coord >= board.size) {
            println!("Coordinate outside bounds of board.");
            continue 'game_loop;
        }

        let coords = (coords[0], coords[1]);

        match board.place_piece(coords) {
            Ok(_) => println!("Placed piece at {}, {}", coords.0, coords.1),
            Err(_) => {
                println!("Cannot place a piece there.");
                continue 'game_loop;
            }
        }

        let win_threshold = board.size;
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

