use crate::{board::Board, player::Player};

pub struct Tournament
{
    yellow_player: Box<dyn Player>,
    blue_player: Box<dyn Player>,
}

impl Tournament
{
    pub fn new(yellow_player: Box<dyn Player>, blue_player: Box<dyn Player>) -> Self {
        Self {
            yellow_player,
            blue_player,
        }
    }

    pub fn play(&self) -> Board {
        let mut board = Board::default();

        loop {
            let selected_move = if board.yellow_turn {
                self.yellow_player.select_move(board)
            } else {
                self.blue_player.select_move(board)
            };

            board = board.play_move(selected_move);

            if board.draw || board.winner.is_some() {
                break;
            }
        }

        board
    }
}
