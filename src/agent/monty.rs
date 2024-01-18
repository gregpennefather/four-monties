use crate::{
    game::board::Board,
    mcst::{node::Node, SearchTree},
};

use super::Agent;

pub struct Monty {
    search_tree: SearchTree,
}

impl Monty {
    pub fn new(board: Board) -> Self {
        Self {
            search_tree: SearchTree::new(board),
        }
    }
}

impl Agent for Monty {
    fn select_move(&mut self, board: Board) -> usize {
        // self.search_tree.print_state();

        for i in 0..200 {
            self.search_tree.iterate();
        }

        // self.search_tree.print_state();
        self.search_tree.choose_move()
    }

    fn record_move(&mut self, index: usize, board: Board) -> Board {
        self.search_tree.record_move(index, board)
    }
}
