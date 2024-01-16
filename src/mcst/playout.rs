use std::fmt::Write;

use crate::board::Board;
use rand::RngCore;

#[derive(Clone, Copy, Debug)]
pub enum PlayoutResult {
    Win,
    Draw,
    Loss,
}

impl std::fmt::Display for PlayoutResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PlayoutResult::Win => "Win",
            PlayoutResult::Loss => "Loss",
            PlayoutResult::Draw => "Draw",
        })
    }
}

pub fn from(mut board: Board) -> PlayoutResult {
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

        if board.draw {
            return PlayoutResult::Draw;
        }

        match board.winner {
            Some(winner) => {
                return if winner != current_player_yellow {
                    PlayoutResult::Loss
                } else {
                    PlayoutResult::Win
                };
            }
            None => continue,
        }
    }
    return PlayoutResult::Draw;
}

impl PlayoutResult {
    pub fn invert(&self) -> Self {
        match self {
            PlayoutResult::Win => PlayoutResult::Loss,
            PlayoutResult::Draw => panic!("Attempting to invert a PlayerResult::Draw"),
            PlayoutResult::Loss => PlayoutResult::Win,
        }
    }

    pub fn fair_result(&self) -> Self {
        match self {
            PlayoutResult::Win => PlayoutResult::Win,
            PlayoutResult::Loss => PlayoutResult::Loss,
            PlayoutResult::Draw => match rand::random() {
                true => PlayoutResult::Win,
                false => PlayoutResult::Loss,
            },
        }
    }
}
