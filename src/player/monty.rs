use crate::{mcst::{SearchTree, node::Node}, board::Board};

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
        for i in 0..100 {
            self.search_tree.iterate(board);
        }

        self.search_tree.select_move()
    }

    fn record_move(&mut self, index: usize, board: Board) -> Board {
        self.search_tree.record_move(index, board)
    }
}