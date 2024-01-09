use crate::{board::Board, tournament::Tournament, player::randy::Randy};
use rand::RngCore;
use colored::Colorize;

mod board;
mod player;
mod tournament;

fn main() {

    let tournament = Tournament::new(Box::new(Randy), Box::new(Randy));

    let board = tournament.play();

    board.print_board();
    println!("Winner: {}",  if board.winner == Some(true) { "Yellow".yellow() } else { "Blue".blue() });
}
