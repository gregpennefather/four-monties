use crate::board::Board;

pub mod monty;
pub mod randy;
pub mod yu;

pub trait Player {
    fn select_move(&mut self, board: Board) -> usize;
    fn record_move(&mut self, index: usize, board: Board) -> Board;
}
