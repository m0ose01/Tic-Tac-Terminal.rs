use std::io;
use crate::board::{
    Square,
    Board,
};

const DIMENSIONS: usize = 2;

fn main() {
    let mut board = Board::new(3);
    let mut turn = 0;
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

        let current_piece =  match turn % 2 == 0 {
            true => Square::X,
            false => Square::O,
        };

        match board.place_piece(current_piece, coords) {
            Ok(_) => println!("Placed piece at {}, {}", coords.0, coords.1),
            Err(_) => {
                println!("Cannot place a piece there.");
                continue 'game_loop;
            }
        }
        turn += 1;
    }
}

mod board {
    use ndarray::{
        Ix2,
        Array,
    };
    use std::fmt;

    #[derive(Clone)]
    pub enum Square {
        X,
        O,
        Blank
    }

    pub struct Board {
        pub size: usize,
        board: Array<Square, Ix2>
    }

    impl Board {
        pub fn new(size: usize) -> Board {
            Board {
                size,
                board: Array::<Square, Ix2>::from_elem((size, size), Square::Blank),
            }
        }

        pub fn place_piece(&mut self, piece: Square, position: (usize, usize)) -> Result<(), ()> {
            let square = &mut self.board[(position.0, position.1)];
            match square {
                Square::Blank => {
                    *square = piece;
                    Ok(())
                },
                _ => Err(())
            }
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let mut string = String::new();
            string.push('-');
            string.push_str(
                &format!("{}\n", "----".repeat(self.size))
            );
            for row in self.board.rows() {
                string.push('|');
                for square in row {
                    string.push_str(
                        &format!(" {square} |")
                    )
                }
                string.push_str(
                    &format!("\n-{}\n", "----".repeat(self.size))
                );
            } 
            write!(formatter, "{string}")
        }
    }

    impl fmt::Display for Square {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let char = match &self {
                Square::X => 'X',
                Square::O => 'O',
                Square::Blank => ' '
            };
            write!(formatter, "{char}")
        }
    }
}

