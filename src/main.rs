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

        let win_threshold = 3;
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

mod board {
    use ndarray::{
        Ix1,
        Ix2,
        Array,
        ArrayView,
    };
    use std::fmt;

    #[derive(Clone, PartialEq)]
    pub enum Square {
        X,
        O,
        Blank
    }

    pub struct Board {
        pub size: usize,
        board: Array<Square, Ix2>,
        turn: usize,
    }

    pub enum BoardStatus {
        None,
        Draw,
        Win(Square),
    }

    impl Board {
        pub fn new(size: usize) -> Board {
            Board {
                size,
                board: Array::<Square, Ix2>::from_elem((size, size), Square::Blank),
                turn: 0,
            }
        }

        pub fn place_piece(&mut self, position: (usize, usize)) -> Result<(), ()> {
            let piece = if self.turn % 2 == 0 {Square::X} else {Square::O};
            let square = &mut self.board[(position.0, position.1)];
            match square {
                Square::Blank => {
                    *square = piece;
                    self.turn += 1;
                    Ok(())
                },
                _ => Err(())
            }
        }

        pub fn check_status(&self, win_threshold: usize) -> BoardStatus {
            let winner = if self.turn % 2 == 0 {Square::O} else {Square::X};

            for current_row in self.board.rows() {
                if check_lane(current_row, win_threshold) {
                    return BoardStatus::Win(winner)
                }
            }

            for current_col in self.board.columns() {
                if check_lane(current_col, win_threshold) {
                    return BoardStatus::Win(winner)
                }
            }

            if self.turn >= usize::pow(self.size, 2) {
                return BoardStatus::Draw;
            }

            return BoardStatus::None
        }
    }

    fn check_lane(lane: ArrayView<Square, Ix1>, win_threshold: usize) -> bool {
        for current_chunk in lane.windows(win_threshold) {
            if current_chunk.iter().any(|x| *x == Square::Blank) {
                continue;
            }
            if current_chunk.iter().all(|x| *x == current_chunk[0]) {
                return true;
            }
        }
        return false
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

