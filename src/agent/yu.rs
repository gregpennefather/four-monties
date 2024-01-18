use std::{io::{self, stdin}, f32::consts::E};

use crate::game::board::Board;

use super::Agent;
use rand::RngCore;

pub struct Yu;

impl Agent for Yu {
    fn select_move(&mut self, board: crate::game::board::Board) -> usize {
        let moves = board.get_moves();
        board.print_board();
        println!("{board:?}");
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

    fn record_move(&mut self, index: usize, board: Board) -> crate::game::board::Board {
        board
    }
}