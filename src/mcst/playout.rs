use std::fmt::Write;

use crate::board::Board;

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

pub fn from(board: Board) -> PlayoutResult {
    PlayoutResult::Draw
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
