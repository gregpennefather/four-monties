use std::default;

use crate::{
    game::{
        board::{Board, WIDTH},
        result::GameResult,
    },
    player::{monty::Monty, randy::Randy, yu::Yu},
    tournament::Tournament,
};
use colored::Colorize;
use rand::RngCore;

mod game;
mod mcst;
mod player;
mod tournament;

fn main() {
    // let b = Board::setup(890452430364, 3507594080739, [0;WIDTH]);
    // // let b = Board::setup(890452430364, 1308570825187, [6,6,6,6,6,6,5]);
    // println!("is_draw: {}", b.draw);
    // b.print_board();
    // return;

    let b = Board::setup(7, 352, Default::default());
    b.print_board();

    let mut yellow_wins = 0;
    let mut blue_wins = 0;
    let mut draws = 0;
    for i in 0..100 {
        let board = Board::default();
        let mut tournament = Tournament::new(Box::new(Yu), Box::new(Yu));

        let board = tournament.play();

        match board.result {
            Some(r) => println!(
                "Game {i} Result: {}",
                match r {
                    GameResult::Draw => {
                        draws += 1;
                        "Draw".to_string()
                    }
                    GameResult::YellowWin => {
                        yellow_wins += 1;
                        "Yellow Wins".yellow().to_string()
                    }
                    GameResult::BlueWin => {
                        blue_wins += 1;
                        "Blue Wins".blue().to_string()
                    }
                }
            ),
            None => (),
        }
    }

    println!(
        "Results {}\\{}\\{}",
        yellow_wins.to_string().yellow(),
        blue_wins.to_string().blue(),
        draws.to_string().bold()
    )
}
