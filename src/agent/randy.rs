use crate::game::board::Board;

use super::Agent;
use rand::RngCore;

pub struct Randy;

impl Agent for Randy {
    fn select_move(&mut self, board: crate::game::board::Board) -> usize {
        let mut rand = rand::thread_rng();

        let moves = board.get_moves();
        let rand_index: usize = rand.next_u64() as usize % moves.len();
        moves[rand_index]
    }

    fn record_move(&mut self, index: usize, board: Board) -> crate::game::board::Board {
        board
    }
}