use crate::board::Board;

mod board;

fn main() {
    let mut board = Board::default();
    board = board.play_move(3);
    println!("{:?}", board.column_pieces);
    println!("{:?}", board.get_moves());
    println!("{:#010b}", board.yellow_bb);
}
