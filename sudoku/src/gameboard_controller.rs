//! GameBoard controller.

use piston::input::GenericEvent;

use crate::GameBoard;

/// Handles events for Sudoku game.
pub struct GameBoardController {
    /// Stores the gameboard state.
    pub gameboard: GameBoard,
}

impl GameBoardController {
    /// Crates a new gameboard controller
    pub fn new(gameboard: GameBoard) -> GameBoardController {
        GameBoardController {
            gameboard: gameboard,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        
    }
}


