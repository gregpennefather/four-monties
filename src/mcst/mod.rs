use std::{
    f32::consts::{E, SQRT_2},
    sync::Arc,
};

use colored::Colorize;
use rand::RngCore;

use crate::{
    game::board::{Board, WIDTH},
    game::result::GameResult,
    mcst::node::insert_to_node_index,
};

use self::node::{ArcNode, Link, Node, NodeContent};

static EXPLORATION_CONSTANT: f32 = SQRT_2;

pub mod node;
mod playout;
mod record;
mod tests;

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
                    let played = c.record.read().unwrap().played as usize;
                    let result = c.result.read().unwrap();

                    println!("Played+Result {i}: {played} - {result:?}", );
                }
                None => println!("{i}: not explored"),
            };
        }
        println!("Current selection: {}", self.select_move());
    }

    pub fn select_move(&self) -> usize {
        let children = self.root.children.read().unwrap();
        let yellow_turn = self.root.board().yellow_turn;
        let mut m: Option<usize> = None;
        let mut m_s = i64::MIN;
        for i in 0..WIDTH {
            let r = match children[i].clone() {
                Some(c) => {
                    // If move is a winner pick it
                    match c.result.try_read() {
                        Ok(r) => {
                            if r.is_some() {
                                match r.unwrap() {
                                    GameResult::YellowWin => if yellow_turn { return i } else { -2 },
                                    GameResult::BlueWin => if !yellow_turn { return i } else { -2 },
                                    GameResult::Draw => 0,
                                }
                            } else {
                                c.record.read().unwrap().played as i64
                            }
                        }
                        Err(e) => panic!("{e}"),
                    }
                }
                None => i64::MIN,
            };
            if r > m_s {
                m = Some(i);
                m_s = r
            }
        }

        if m.is_none() {
            self.root.board().print_board();
            panic!("no valid move found for node {:?}", self.root);
        }
        m.unwrap()

    }

    pub fn iterate(&mut self) {
        let selection = self.selection();

        if selection.is_none() {
            println!("No expansion for root {:?}", self.root);
            return;
        }

        let new_leaf = self.expansion(selection.clone().unwrap());
        let sim_result = self.simulation(new_leaf.clone());
        backpropagation(new_leaf.clone(), sim_result);
    }

    pub fn selection(&self) -> Option<ArcNode> {
        let root = self.root.clone();
        let root_sims = root.record.read().unwrap().played as f32;
        let (selected, score) = traverse_tree_ucb(root.clone(), root_sims, 0);

        // println!("Selected {selected:?} with score {score}");

        if selected.is_none() {
            println!("No valid expansion for root {root:?}");
            return None;
        }

        if !selected.clone().unwrap().is_leaf() {
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
        selected
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
        match new_arc_node.result.try_read() {
            Ok(r) => {
                if r.is_some() {
                    if r.unwrap() == GameResult::YellowWin && leaf.board().yellow_turn || r.unwrap() == GameResult::BlueWin && !leaf.board().yellow_turn {
                        match leaf.result.try_write() {
                            Ok(mut w) => *w = *r,
                            Err(e) => panic!("{e}")
                        }
                    }
                }
            }
            Err(e) => panic!("{e}"),
        }

        new_arc_node.clone()
    }

    pub fn simulation(&mut self, leaf: ArcNode) -> GameResult {
        let board = leaf.clone().board();
        playout::from(board).fair_result()
    }
}

pub fn backpropagation(mut leaf: ArcNode, result: GameResult) {
    leaf.record_result(result);
    match leaf.parent.upgrade() {
        Some(l) => backpropagation(l, result),
        None => {}
    }
}

fn traverse_tree_ucb(node: ArcNode, parent_sims: f32, depth: usize) -> (Option<ArcNode>, f32) {
    if node.clone().board().result.is_some() {
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
        game::board::Board,
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
