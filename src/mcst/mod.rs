use std::{
    f32::consts::{E, SQRT_2},
    sync::Arc,
};

use colored::Colorize;
use rand::RngCore;

use crate::{
    board::{Board, WIDTH},
    mcst::node::insert_to_node_index,
};

use self::{
    node::{ArcNode, Link, Node, NodeContent},
    playout::PlayoutResult,
};

static EXPLORATION_CONSTANT: f32 = SQRT_2;

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
            None => insert_to_node_index(&mut self.root, index, board).clone(),
        };

        self.root = new_root.clone();
        board
    }

    pub fn print_state(&self) {
        let children = self.root.children.read().unwrap();
        for i in 0..WIDTH {
            let r = match children[i].clone() {
                Some(c) => {
                    // Else rank moves by simulation count
                    let r = c.record.read().unwrap().played as usize;

                    println!("{i}: {r} - {:?}", c.board.winner);
                }
                None => println!("{i}: not explored"),
            };
        }
        println!("Current selection: {}", self.select_move());
    }

    pub fn select_move(&self) -> usize {
        let children = self.root.children.read().unwrap();
        let mut m = 0;
        let mut m_s = 0;
        for i in 0..WIDTH {
            let r = match children[i].clone() {
                Some(c) => {
                    // If move is a winner pick it
                    if c.board().winner == Some(self.root.board.yellow_turn) {
                        return i;
                    }
                    // Else rank moves by simulation count
                    c.record.read().unwrap().played as usize
                }
                None => 0,
            };
            if r > m_s {
                m = i;
                m_s = r
            }
        }
        m
    }

    pub fn iterate(&mut self, board_state: Board) {
        // let leaf = self.root.select();
        if self.root.clone().board() != board_state {
            self.root.board().print_board();
            board_state.print_board();
            panic!("Unexpected state!");
        }

        let selection = self.selection();
        let new_leaf = self.expansion(selection.clone());
        let sim_result = self.simulation(new_leaf.clone());
        backpropagation(new_leaf.clone(), sim_result);
    }

    pub fn selection(&self) -> ArcNode {
        let root = self.root.clone();
        let root_sims = root.record.read().unwrap().played as f32;
        let (selected, score) = traverse_tree_ucb(root.clone(), root_sims, 0);

        if selected.is_none() {
            println!("No valid expansion for root {root:?}");
        }

        if !selected.clone().unwrap().is_leaf() {
            println!("root {root:?}");
            let children = root.children.read();
            println!("{children:?}");
            root.board().print_board();
            println!(
                "{} {} : {:?}",
                root.board.yellow_bb.to_string().yellow(),
                root.board.blue_bb.to_string().blue(),
                root.board.column_pieces
            );
            panic!("Selected is not leaf! {selected:?} score {score}");
        }
        match selected {
            Some(n) => n,
            None => self.root.clone(),
        }
    }

    pub fn expansion(&mut self, mut leaf: ArcNode) -> ArcNode {
        let mut rand = rand::thread_rng();
        let moves = leaf.board().get_moves();
        let options: Vec<usize> = leaf
            .get_uninitialized_children()
            .into_iter()
            .filter(|o| moves.contains(o))
            .collect();

        if options.len() == 0 {
            panic!("No valid options for expansion");
        }

        let rand_index: usize = rand.next_u64() as usize % options.len();

        let selected_move = options[rand_index];
        let new_state = leaf.board().play_move(selected_move);
        let new_arc_node = insert_to_node_index(&mut leaf, selected_move, new_state);

        new_arc_node.clone()
    }

    pub fn simulation(&mut self, leaf: ArcNode) -> PlayoutResult {
        let board = leaf.clone().board();
        if let Some(winner) = board.winner {
            return if winner == board.yellow_turn {
                PlayoutResult::Win
            } else {
                PlayoutResult::Loss
            };
        }
        if board.draw {
            return PlayoutResult::Draw.fair_result();
        }
        playout::from(board).fair_result()
    }
}

pub fn backpropagation(mut leaf: ArcNode, result: PlayoutResult) {
    leaf.record_result(result);
    match leaf.parent.upgrade() {
        Some(l) => backpropagation(l, result.invert()),
        None => {}
    }
}

fn traverse_tree_ucb(node: ArcNode, parent_sims: f32, depth: usize) -> (Option<ArcNode>, f32) {
    if node.clone().board().complete {
        (None, f32::MIN)
    } else if node.clone().is_leaf() {
        (
            Some(node.clone()),
            calculate_node_uctb(node.clone(), parent_sims),
        )
    } else {
        let children = node.children.read().unwrap();
        let sims = node.record.read().unwrap().played as f32;
        let mut max_score = f32::MIN;
        let mut selected_node: Option<ArcNode> = None;
        for i in 0..WIDTH {
            let child = children[i].clone();
            if child.is_some() {
                let (selected, r) = traverse_tree_ucb(child.unwrap(), sims, depth + 1);

                if r > max_score {
                    max_score = r;
                    selected_node = selected.clone();
                }
            }
        }

        // return max of
        (selected_node, max_score)
    }
}

fn calculate_node_uctb(node: ArcNode, parent_sims: f32) -> f32 {
    match node.clone().record.try_read() {
        Ok(r) => {
            let mean = r.wins as f32 / r.played as f32;
            let exploration_bias =
                EXPLORATION_CONSTANT * f32::sqrt(f32::ln(parent_sims) / r.played as f32);
            mean + exploration_bias
        }
        Err(e) => panic!("Calculate node uctb error: {e}"),
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
