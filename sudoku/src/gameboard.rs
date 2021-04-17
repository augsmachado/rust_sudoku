//! Game board logic.

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
pub struct GameBoard {
    /// Stores the content of the cells.
    /// `0` is an empty cells.
    pub cells: [[u8; SIZE]; SIZE];
}

impl GameBoard {
    /// Creates a new game board.
    pub fn new() -> GameBoard {
        GameBoard {
            cells: [[0; SIZE]; SIZE],
        }
    }
}