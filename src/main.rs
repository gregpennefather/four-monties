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
    let mut yellow_wins = 0;
    let mut yellow_win_aggregate_turns = 0;
    let mut blue_wins = 0;
    let mut blue_win_aggregate_turns = 0;
    let mut draws = 0;
    for i in 0..500 {
        let board = Board::default();
        let mut tournament = Tournament::new(Box::new(Monty::new(board)), Box::new(Randy));

        let board = tournament.play();

        match board.result {
            Some(r) => println!(
                "Game {i} Result: {} (Turn {})",
                match r {
                    GameResult::Draw => {
                        draws += 1;
                        "Draw".to_string()
                    }
                    GameResult::YellowWin => {
                        yellow_wins += 1;
                        yellow_win_aggregate_turns += board.turn;
                        "Yellow Wins".yellow().to_string()
                    }
                    GameResult::BlueWin => {
                        blue_wins += 1;
                        blue_win_aggregate_turns += board.turn;
                        "Blue Wins".blue().to_string()
                    }
                },
                board.turn
            ),
            None => (),
        }
    }

    println!(
        "Results {}\\{}\\{}",
        yellow_wins.to_string().yellow(),
        blue_wins.to_string().blue(),
        draws.to_string().bold()
    );
    println!("Average yellow win turn {}", yellow_win_aggregate_turns / yellow_wins);
    println!("Average blue win turn {}", blue_win_aggregate_turns / blue_wins);
}
