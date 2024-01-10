use std::sync::Arc;

use rand::RngCore;

use crate::{board::Board, mcst::node::insert_to_node_index};

use self::{node::{ArcNode, Link, Node, NodeContent}, playout::PlayoutResult};

pub mod node;
mod playout;
mod record;

pub struct SearchTree {
    pub root: ArcNode,
}

impl SearchTree {
    pub fn new(board: Board) -> Self {
        Self {
            root: Arc::new(NodeContent::new_root(board)),
        }
    }

    pub fn record_move(&mut self, index: usize, board: Board) -> Board {
        let root = self.root.clone();
        let children = root.children.read().unwrap();
        let child = children[index].clone();
        drop(children);
        let new_root = match child {
            Some(c) => c.clone(),
            None => insert_to_node_index(&mut self.root, index, board).clone()
        };

        self.root = new_root.clone();
        board
    }

    pub fn produce_move(&mut self, board_state: Board) {
        // let leaf = self.root.select();
        let root = self.root.clone().seek(board_state);
        let new_leaf = self.expansion(root.clone().unwrap().board());
        let sim_result = self.simulation(new_leaf.clone());
        println!("sim_result: {sim_result}");
        backpropagation(new_leaf.clone(), sim_result);
        println!("{}", new_leaf.record.read().unwrap())
    }

    pub fn expansion(&mut self, board: Board) -> ArcNode {
        let root = self.root.clone();
        let mut leaf = root.seek(board).unwrap();

        let mut rand = rand::thread_rng();
        let moves = leaf.board().get_moves();
        let rand_index: usize = rand.next_u64() as usize % moves.len();

        let selected_move = moves[rand_index];
        let new_arc_node = insert_to_node_index(
            &mut leaf,
            selected_move,
            board.play_move(selected_move),
        );

        new_arc_node.clone()
    }

    pub fn simulation(&mut self, leaf: ArcNode) -> PlayoutResult {
        let board = leaf.clone().board();
        playout::from(board).fair_result()
    }
}


pub fn backpropagation(leaf: ArcNode, result: PlayoutResult) {
    leaf.clone().record_result(result);
    match leaf.parent.upgrade() {
        Some(l) => backpropagation(l, result.invert()),
        None => {}
    }
}

#[cfg(test)]
mod test {
    use crate::{
        board::Board,
        mcst::{
            node::{ArcNode, NodeContent},
            SearchTree,
        },
    };

    #[test]
    pub fn insert_to_tree_root() {
        // Act
        let tree = SearchTree::new(Board::default());

        // Assert
        assert_eq!(tree.root.board, Board::default());
    }
}
