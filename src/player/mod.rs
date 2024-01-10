use crate::board::Board;

pub mod randy;
pub mod monty;

pub trait Player {
    fn select_move(&mut self, board: Board) -> usize;
}
