use crate::{
    board::Board,
    player::{monty::Monty, randy::Randy},
    tournament::Tournament,
};
use colored::Colorize;
use rand::RngCore;

mod board;
mod mcst;
mod player;
mod tournament;

fn main() {
    let board = Board::default();
    let mut tournament = Tournament::new(Box::new(Monty::new(board)), Box::new(Randy));

    let board = tournament.play();

    board.print_board();
    println!(
        "Winner: {}",
        if board.winner == Some(true) {
            "Yellow".yellow()
        } else {
            "Blue".blue()
        }
    );
}
