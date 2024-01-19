use std::default;

use crate::{
    game::{
        board::{Board, WIDTH},
        result::GameResult, player::Player,
    },
    agent::{monty::Monty, randy::Randy, yu::Yu},
    tournament::Tournament, mcst::SearchTree,
};
use colored::Colorize;
use rand::RngCore;

mod game;
mod mcst;
mod agent;
mod tournament;

fn main() {
    let mut yellow_wins = 0;
    let mut yellow_win_aggregate_turns = 0;
    let mut blue_wins = 0;
    let mut blue_win_aggregate_turns = 0;
    let mut draws = 0;
    for i in 0..20 {
        let board = Board::default();
        let mut tournament = Tournament::new(Box::new(Monty::new(board, 50, 50)), Box::new(Monty::new(board, 100, 50)));

        let board = tournament.play();

        match board.winner {
            Some(r) => println!(
                "Game {i} Result: {} (Turn {})",
                match r {
                    Player::NoPlayer => {
                        draws += 1;
                        board.print_board();
                        "Draw".to_string()
                    }
                    Player::Yellow => {
                        yellow_wins += 1;
                        yellow_win_aggregate_turns += board.turn;
                        "Yellow Wins".yellow().to_string()
                    }
                    Player::Blue => {
                        blue_wins += 1;
                        blue_win_aggregate_turns += board.turn;
                        board.print_board();
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
