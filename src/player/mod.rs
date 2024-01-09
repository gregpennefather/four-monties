use crate::board::Board;

mod randy;

pub trait Player {
    fn select_move(board: Board) -> usize;
}
