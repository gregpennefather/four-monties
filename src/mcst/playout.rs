use std::fmt::Write;

use crate::game::{board::Board, result::GameResult};
use rand::RngCore;

pub fn from(mut board: Board) -> GameResult {
    if let Some(r) = board.winner {
        return GameResult::Win(r)
    }
    if board.get_moves().len() == 0 {
        println!("Trying to simulate state with no moves: {board:?}");
    }
    let mut rand = rand::thread_rng();
    for i in 0..1000 {
        let moves = board.get_moves();
        if board.get_moves().len() == 0 {
            println!("depth {i} board: {board:?}");
            board.print_board();
        }


        let rand_index: usize = rand.next_u64() as usize % moves.len();
        board = board.play_move(moves[rand_index]);

        match board.winner {
            Some(result) => return GameResult::Win(result).fair_random_result(),
            None => continue,
        }
    }
    return GameResult::Draw;
}