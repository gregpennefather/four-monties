use std::{io::{self, stdin}, f32::consts::E};

use crate::board::Board;

use super::Player;
use rand::RngCore;

pub struct Yu;

impl Player for Yu {
    fn select_move(&mut self, board: crate::board::Board) -> usize {
        let moves = board.get_moves();
        board.print_board();
        println!("Select move : {moves:?}");
        let mut entry = String::new();
        loop {
            stdin().read_line(&mut entry);
            match entry.trim().parse() {
                Ok(r) => return r,
                Err(e) => println!("Invalid move please select another")
            }
        }
    }

    fn record_move(&mut self, index: usize, board: Board) -> crate::board::Board {
        board
    }
}