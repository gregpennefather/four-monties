use crate::{mcst::SearchTree, board::Board};

use super::Player;

pub struct Monty {
    search_tree: SearchTree
}

impl Monty {
    pub fn new(board: Board) -> Self {
        Self {
            search_tree: SearchTree::new(board)
        }
    }
}

impl Player for Monty {
    fn select_move(&mut self, board: Board) -> usize {
        self.search_tree.produce_move(board);
        let moves = board.get_moves();
        moves[0]
    }

    fn record_move(&mut self, index: usize, board: Board) -> Board {
        self.search_tree.record_move(index, board)
    }
}