use crate::board::Board;

pub mod randy;

pub trait Player {
    fn select_move(&self, board: Board) -> usize;
}
