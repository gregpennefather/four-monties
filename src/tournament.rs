use colored::Colorize;

use crate::{agent::Agent, game::{board::Board, player::Player}};

pub struct Tournament {
    yellow_player: Box<dyn Agent>,
    blue_player: Box<dyn Agent>,
}

impl Tournament {
    pub fn new(yellow_player: Box<dyn Agent>, blue_player: Box<dyn Agent>) -> Self {
        Self {
            yellow_player,
            blue_player,
        }
    }

    pub fn play(&mut self) -> Board {
        let mut board = Board::default();
        let mut m_count = 0;

        loop {
            let selected_move = if board.active_player == Player::Yellow {
                self.yellow_player.select_move(board)
            } else {
                self.blue_player.select_move(board)
            };

            board = board.play_move(selected_move);
            self.yellow_player.record_move(selected_move, board);
            self.blue_player.record_move(selected_move, board);

            // Active player has inverted at this point
            m_count += 1;
            // println!(
            //     "Move {m_count}: {}",
            //     if !board.yellow_turn {
            //         selected_move.to_string().yellow()
            //     } else {
            //         selected_move.to_string().blue()
            //     }
            // );

            if board.winner.is_some() {
                break;
            }
        }

        board
    }
}
