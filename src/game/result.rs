use core::fmt;

use super::player::Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameResult {
    Win(Player),
    Draw,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GameResult {
    pub fn fair_random_result(&self) -> Self {
        match self {
            GameResult::Win(Player::Yellow) => GameResult::Win(Player::Yellow),
            GameResult::Win(Player::Blue) => GameResult::Win(Player::Blue),
            GameResult::Win(Player::NoPlayer) | GameResult::Draw => match rand::random() {
                true => GameResult::Win(Player::Yellow),
                false => GameResult::Win(Player::Blue),
            },
        }
    }
}
