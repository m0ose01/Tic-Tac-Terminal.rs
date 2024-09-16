use ndarray::{
    Ix1,
    Ix2,
    Array,
    ArrayView,
    Axis,
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

        let diagonal_count = 1 + (self.size - win_threshold) * 2;

        for current_diag_offset in 0..diagonal_count {
            let offset_a = (0, current_diag_offset);
            let offset_b = (current_diag_offset, 0);
            if check_lane(get_diagonal(self.board.view(), offset_a).expect("Invalid offset."), win_threshold) {
                return BoardStatus::Win(winner);
            }
            if check_lane(get_diagonal(self.board.view(), offset_b).expect("Invalid offset."), win_threshold) {
                return BoardStatus::Win(winner);
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

fn get_diagonal(array: ArrayView<Square, Ix2>, offset: (usize, usize)) -> Result<ArrayView<Square, Ix1>, (usize, usize)> {
    if !((offset.0 == 0) | (offset.1 == 0)) {
        return Err(offset);
    }
    let diagonal_length = array.shape()[0] - (offset.0 + offset.1);

    // One offset must be zero, so this effectively "selects" the non-zero offset.
    let offset_sum = offset.0 + offset.1;
    Ok(
        if offset.1 > offset.0 {
            let horizontal_split = offset_sum;
            let vertical_split = diagonal_length;
            array
                .split_at(Axis(0), horizontal_split).1
                .split_at(Axis(1), vertical_split).0
                .into_diag()
        } else {
            let horizontal_split = diagonal_length;
            let vertical_split = offset_sum;
            array
                .split_at(Axis(0), horizontal_split).0
                .split_at(Axis(1), vertical_split).1
                .into_diag()
        }
    )
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

