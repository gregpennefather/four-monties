use crate::board::Board;
use rand::RngCore;
use colored::Colorize;

mod board;
mod player;

fn main() {
    let mut board = Board::default();
    let mut rand = rand::thread_rng();
    loop {
        let moves = board.get_moves();
        let rand_index: usize = rand.next_u64() as usize % moves.len();
        board.print_board();
        println!("{} \tColumn Selected: {}", if board.yellow_turn { "Yellow".yellow() } else { "Blue".blue() },  moves[rand_index]);
        board = board.play_move(moves[rand_index]);
        if board.winner.is_some() {
            break;
        }
    }
    board.print_board();
    println!("Winner: {}",  if board.winner == Some(true) { "Yellow".yellow() } else { "Blue".blue() });
    println!("bb : {}", if board.winner == Some(true) { board.yellow_bb } else { board.blue_bb })
}
