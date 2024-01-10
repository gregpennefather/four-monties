use colored::Colorize;

use crate::{board::Board, player::Player};

pub struct Tournament {
    yellow_player: Box<dyn Player>,
    blue_player: Box<dyn Player>,
}

impl Tournament {
    pub fn new(yellow_player: Box<dyn Player>, blue_player: Box<dyn Player>) -> Self {
        Self {
            yellow_player,
            blue_player,
        }
    }

    pub fn play(&mut self) -> Board {
        let mut board = Board::default();
        let mut m_count = 0;

        loop {
            let selected_move = if board.yellow_turn {
                self.yellow_player.select_move(board)
            } else {
                self.blue_player.select_move(board)
            };

            println!("Move {} selected!", m_count + 1);

            board = board.play_move(selected_move);
            self.yellow_player.record_move(selected_move, board);
            self.blue_player.record_move(selected_move, board);

            // Active player has inverted at this point
            m_count += 1;
            println!(
                "Move {m_count}: {}",
                if !board.yellow_turn {
                    selected_move.to_string().yellow()
                } else {
                    selected_move.to_string().blue()
                }
            );

            if board.draw || board.winner.is_some() {
                break;
            }
        }

        board
    }
}
