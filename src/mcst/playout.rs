use std::fmt::Write;

use crate::game::{board::Board, result::GameResult};
use rand::RngCore;

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameResult::YellowWin => "Yellow",
            GameResult::BlueWin => "Blue",
            GameResult::Draw => "Draw",
        })
    }
}

pub fn from(mut board: Board) -> GameResult {
    if board.get_moves().len() == 0 {
        println!("Trying to simulate state with no moves: {board:?}");
    }
    let mut rand = rand::thread_rng();
    let current_player_yellow = board.yellow_turn;
    for i in 0..1000 {
        let moves = board.get_moves();
        if board.get_moves().len() == 0 {
            println!("depth {i} board: {board:?}");
            board.print_board();
        }


        let rand_index: usize = rand.next_u64() as usize % moves.len();
        board = board.play_move(moves[rand_index]);

        match board.result {
            Some(result) => return result,
            None => continue,
        }
    }
    return GameResult::Draw;
}