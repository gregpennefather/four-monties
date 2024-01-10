use crate::board::Board;

use super::Player;
use rand::RngCore;

pub struct Randy;

impl Player for Randy {
    fn select_move(&mut self, board: crate::board::Board) -> usize {
        let mut rand = rand::thread_rng();

        let moves = board.get_moves();
        let rand_index: usize = rand.next_u64() as usize % moves.len();
        moves[rand_index]
    }

    fn record_move(&mut self, index: usize, board: Board) -> crate::board::Board {
        board
    }
}