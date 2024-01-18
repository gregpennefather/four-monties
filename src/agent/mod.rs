use crate::game::board::Board;

pub mod monty;
pub mod randy;
pub mod yu;

pub trait Agent {
    fn select_move(&mut self, board: Board) -> usize;
    fn record_move(&mut self, index: usize, board: Board) -> Board;
}
