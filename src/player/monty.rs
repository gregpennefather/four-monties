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
    fn select_move(&mut self, board: crate::board::Board) -> usize {
        self.search_tree.expansion(board);
        let moves = board.get_moves();
        moves[0]
    }
}