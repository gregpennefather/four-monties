use crate::{
    board::{Board, WIDTH},
    player::{monty::Monty, randy::Randy, yu::Yu},
    tournament::Tournament,
};
use colored::Colorize;
use rand::RngCore;

mod board;
mod mcst;
mod player;
mod tournament;

fn main() {
    // let b = Board::setup(890452430364, 3507594080739, [0;WIDTH]);
    // // let b = Board::setup(890452430364, 1308570825187, [6,6,6,6,6,6,5]);
    // println!("is_draw: {}", b.draw);
    // b.print_board();
    // return;

    let mut yellow_wins = 0;
    let mut blue_wins = 0;
    let mut draws = 0;
    for i in 0..1000 {
        let board = Board::default();
        let mut tournament = Tournament::new(Box::new(Monty::new(board)), Box::new(Randy));

        let board = tournament.play();

        if board.draw {
            draws += 1;
        } else if board.winner == Some(true) {
            yellow_wins += 1;
        } else {
            blue_wins += 1
        }
        println!(
            "Game {i} Winner: {}",
            if board.winner == Some(true) {
                "Yellow".yellow()
            } else {
                "Blue".blue()
            }
        );
    }

    println!("Results {}\\{}\\{}", yellow_wins.to_string().yellow(), blue_wins.to_string().blue(), draws.to_string().bold())
}
