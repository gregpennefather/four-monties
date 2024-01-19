use std::{
    f32::consts::{E, SQRT_2},
    sync::{Arc, RwLock},
};

use colored::Colorize;
use log::debug;
use rand::RngCore;

use crate::{
    game::board::{Board, WIDTH},
    game::result::GameResult,
};

use self::{
    node::{ActionLink, ArcNode, Node, NodeContent},
    valid_move::ValidMove,
};

static EXPLORATION_CONSTANT: f32 = SQRT_2;

pub mod node;
mod playout;
mod record;
mod tests;
mod valid_move;

pub struct SearchTree {
    pub root: ArcNode,
    simulations: usize,
}

impl SearchTree {
    pub fn new(board: Board, simulations: usize) -> Self {
        Self {
            root: Arc::new(NodeContent::new_root(board)),
            simulations,
        }
    }

    pub fn record_move(&mut self, index: usize, board: Board) -> Board {
        let root = self.root.clone();
        let children = match root.children.get() {
            Some(children) => children,
            None => {
                debug!("Attempting to record move {index} but children haven't been init'd for node {:?}", self.root); // TODO: This shouldnt be happening as often as it is
                self.expansion(root.clone());
                root.children.get().unwrap()
            }
        };
        let new_root = match &children[index] {
            ValidMove::Valid(c) => c.clone(),

            ValidMove::Invalid => {
                panic!("Something went wrong - attempting to record an invalid move")
            }
        };

        self.root = new_root.clone();
        board
    }

    pub fn print_state(&self) {
        println!("State winner: {:?}", self.root.result.get());
        match self.root.children.get() {
            Some(children) => {
                for i in 0..WIDTH {
                    match &children[i] {
                        ValidMove::Valid(c) => {
                            // Else rank moves by simulation count
                            let wins = c.record.read().unwrap().wins as usize;
                            let played = c.record.read().unwrap().played as usize;
                            let result = c.result.get();

                            println!("Option {i}: {wins}\\{played} - {result:?}",);
                        }
                        ValidMove::Invalid => println!("{i}: not valid"),
                    };
                }
                println!("Expected move: {}", self.choose_move());
            }
            None => println!("Unexplored root"),
        }
    }

    pub fn choose_move(&self) -> usize {
        match self.root.children.get() {
            Some(children) => {
                let mut m: Option<usize> = None;
                let mut m_s = i64::MIN;
                for i in 0..WIDTH {
                    let r = match &children[i] {
                        ValidMove::Valid(c) => {
                            // If move is a winner pick it
                            let r = c.result.get();
                            match r {
                                Some(r) => match r {
                                    GameResult::Win(winner) => {
                                        if *winner == self.root.board().active_player {
                                            return i;
                                        } else {
                                            -2
                                        }
                                    }
                                    GameResult::Draw => 0,
                                },
                                None => c.record.read().unwrap().played as i64,
                            }
                        }
                        ValidMove::Invalid => i64::MIN,
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
            None => panic!("Attempting to choose move when root has no children"),
        }
    }

    pub fn iterate(&mut self) {
        // Game over no need to iterate
        if self.root.result.get().is_some() {
            return;
        }

        let selection = self.selection();

        if selection.is_none() {
            debug!("No expansion for root {:?}", self.root);
            return;
        }

        self.expansion(selection.clone().unwrap());
        match selection.clone().unwrap().children.get() {
            Some(children) => {
                for i in 0..WIDTH {
                    match &children[i] {
                        ValidMove::Valid(m) => {
                            for i in 0..self.simulations {
                                let sim_result = self.simulation(m.clone());
                                backpropagation(m.clone(), sim_result);
                            }
                        }
                        ValidMove::Invalid => (),
                    }
                }
            }
            None => (),
        }
    }

    pub fn selection(&self) -> Option<ArcNode> {
        let root = self.root.clone();
        let root_sims = root.record.read().unwrap().played as f32;
        let (selected, score) = traverse_tree_ucb(root.clone(), root_sims, 0);

        // println!("Selected {selected:?} with score {score}");

        if selected.is_none() {
            debug!("No valid expansion for root {root:?}");
            return None;
        }

        if !selected.clone().unwrap().is_leaf() {
            match root.children.get() {
                Some(children) => {
                    println!("{children:?}");
                }
                None => (),
            }
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

    pub fn expansion(&mut self, mut leaf: ArcNode) {
        let moves = leaf.board().get_moves();

        let mut new_leaves: [ActionLink; WIDTH] = std::array::from_fn(|_| ValidMove::Invalid);

        for selected_move in 0..WIDTH {
            if moves.contains(&selected_move) {
                let new_state = leaf.board().play_move(selected_move);
                let new_arc_node = leaf.new_child(selected_move, new_state);
                new_leaves[selected_move] = ValidMove::Valid(new_arc_node.clone());

                match new_arc_node.result.get() {
                    Some(r) => {
                        match r {
                            GameResult::Win(winner) => {
                                if *winner == leaf.board().active_player {
                                    match leaf.result.set(*r) {
                                        Err(val) => {
                                            if Some(&val) != leaf.result.get() {
                                                panic!("Could not write result to {:?}. Write Value: {val}", leaf)
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            GameResult::Draw => (), // TODO: Maybe we need to backpropagate draws?
                        }
                    }
                    None => (),
                }
            }
        }
        leaf.children.set(new_leaves);
    }

    pub fn simulation(&mut self, leaf: ArcNode) -> GameResult {
        let board = leaf.clone().board();
        playout::from(board).fair_random_result()
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
    if node.clone().board().winner.is_some() {
        (None, f32::MIN)
    } else if node.clone().is_leaf() {
        (
            Some(node.clone()),
            calculate_node_uctb(node.clone(), parent_sims),
        )
    } else {
        match node.children.get() {
            Some(children) => {
                let sims = node.record.read().unwrap().played as f32;
                let mut max_score = f32::MIN;
                let mut selected_node: Option<ArcNode> = None;
                for i in 0..WIDTH {
                    if let ValidMove::Valid(child) = &children[i] {
                        let (selected, r) = traverse_tree_ucb(child.clone(), sims, depth + 1);

                        if r > max_score {
                            max_score = r;
                            selected_node = selected.clone();
                        }
                    }
                }

                // return max of
                (selected_node, max_score)
            }
            None => panic!("Trying to find children of childless node {:?}", node),
        }
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
        let tree = SearchTree::new(Board::default(), 10);

        // Assert
        assert_eq!(tree.root.board, Board::default());
    }
}
