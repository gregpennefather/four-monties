use crate::board::Board;
use rand::RngCore;
use colored::Colorize;

mod board;
mod player;

fn main() {
    let mut board = Board::default();
    let mut rand = rand::thread_rng();

    board = board.play_move(0);
    board = board.play_move(0);
    board = board.play_move(0);
    board = board.play_move(0);
    board = board.play_move(0);
    board = board.play_move(0);

    board.print_board();

    println!("{:?}", board.get_moves());

    println!("Winner: {}",  if board.winner == Some(true) { "Yellow".yellow() } else { "Blue".blue() });
    println!("bb : {}", if board.winner == Some(true) { board.yellow_bb } else { board.blue_bb })
}
