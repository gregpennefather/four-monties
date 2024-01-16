use core::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock, Weak};

use crate::board::{Board, WIDTH};

use super::{playout::PlayoutResult, record::Record};

pub struct NodeContent {
    pub board: Board,
    pub parent: Weak<Self>,
    pub record: RwLock<Record>,
    pub children: RwLock<[Link; 7]>,
}

impl NodeContent {
    pub(super) fn new_root(board: Board) -> Self {
        NodeContent {
            board: board,
            parent: Weak::new(),
            record: Default::default(),
            children: RwLock::new(Default::default()),
        }
    }
    pub(super) fn new_child(parent_ptr: Weak<Self>, board: Board) -> Self {
        NodeContent {
            board: board,
            parent: parent_ptr,
            record: Default::default(),
            children: RwLock::new(Default::default()),
        }
    }

    pub fn is_leaf(&self) -> bool {
        let moves = self.board.get_moves();
        let children = self.children.read().unwrap();
        for i in 0..WIDTH {
            if children[i].is_none() && moves.contains(&i) {
                return true;
            }
        }
        false
    }

    pub fn get_uninitialized_children(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        let children = self.children.read().unwrap();
        for i in 0..WIDTH {
            if children[i].is_none() {
                vec.push(i)
            }
        }
        vec
    }
}

impl Debug for NodeContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node")
            .field(&self.board)
            .field(&self.record.read())
            .field(&self.is_leaf())
            .finish()
    }
}

pub type ArcNode = Arc<NodeContent>;
pub type Link = Option<ArcNode>;

// pub struct Tree {
//     root: Link,
// }

// impl Tree {
//     pub fn new(root: Link) -> Self {
//         Self { root }
//     }
// }

pub trait Node {
    fn board(&self) -> Board;
    fn record_result(&mut self, result: PlayoutResult);
    fn new_child(&self, index: usize, board: Board) -> Self;
    fn seek(self, board: Board) -> Option<Self>
    where
        Self: Sized;
}

pub fn insert_to_node_index(
    root: &mut ArcNode,
    index: usize,
    board: Board,
) -> ArcNode {
    let child = Some(root.new_child(index, board));
    match root.children.try_write() {
        Ok(mut children) => children[index] = child.clone(),
        Err(e) => panic!("Err {e:?}!"),
    }
    child.unwrap()
}

impl Node for ArcNode {
    fn board(&self) -> Board {
        self.board
    }

    fn record_result(&mut self, result: PlayoutResult) {
        match self.record.try_write() {
            Ok(mut r) => r.increment(result),
            Err(e) => panic!("Record result lock error {e:?}"),
        }
    }

    fn new_child(&self, index: usize, board: Board) -> Self {
        let lock = self.children.read();
        if lock.unwrap()[index].is_some() {
            panic!("Attempting to insert over existing node");
        }
        let parent_ptr = Arc::downgrade(self);
        Arc::new(NodeContent::new_child(parent_ptr, board))
    }

    fn seek(self, board: Board) -> Option<Self> {
        if self.board == board {
            return Some(self.clone());
        } else {
            for i in 0..WIDTH {
                match &self.children.read().unwrap()[i] {
                    Some(s) => {
                        let n = s.clone().seek(board);
                        match n {
                            Some(tn) => return Some(tn),
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }
        }

        None
    }

    // fn new(board: Board, parent: Option<Weak<Self>>) -> Self { // , parent: Option<Rc<Node>>
    //     Self {
    //         board,
    //         parent,
    //         children: Default::default(),
    //     }
    // }

    // pub fn insert(&mut self, index: usize, board: Board, r: Rc<RefCell<Self>>) {
    //     if self.children[index].is_some() {
    //         panic!("Attempting to insert over existing node");
    //     }
    //     let parent_ptr = Rc::downgrade(&r);
    //     let new_node = Rc::new(RefCell::new(Node::new(board, Some(parent_ptr))));

    //     self.children[index] = Some(new_node);
    // }
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    pub fn insert_to_node_first_position() {
        // Arrange
        let mut root = ArcNode::new(NodeContent::new_root(Board::default()));

        // Act
        insert_to_node_index(&mut root, 0, Board::default());

        // Assert
        let lock = root.children.read();
        assert!(lock.unwrap()[0].is_some())
    }

    #[test]
    pub fn find_child_on_root_returns_root() {
        // Arrange
        let root = Some(ArcNode::new(NodeContent::new_root(Board::default())));

        // Act
        let seek = root.unwrap().seek(Board::default());

        // Assert
        assert!(seek.is_some());
    }

    #[test]
    pub fn find_child_not_root() {
        // Arrange
        let board1 = Board::setup(1, 0, [0;WIDTH]);
        let board2 = Board::setup(2, 0, [0;WIDTH]);
        let root = Some(ArcNode::new(NodeContent::new_root(Board::default())));

        insert_to_node_index(&mut root.clone().unwrap().clone(), 0, board1);
        insert_to_node_index(&mut root.clone().unwrap().clone(), 1, board2);

        // Act
        let seek = root.unwrap().seek(board2);

        // Assert
        assert!(seek.is_some());
        assert_eq!(seek.as_ref().unwrap().board, board2)
    }

    #[test]
    pub fn is_leaf_all_children_assigned() {
        // Arrange
        let board = Board::default();
        let root = Some(ArcNode::new(NodeContent::new_root(board)));

        for i in 0..WIDTH {
            insert_to_node_index(&mut root.clone().unwrap().clone(), i, board.play_move(i));
        }
        // Assert
        assert!(!root.unwrap().is_leaf())
    }

    #[test]
    pub fn is_leaf_no_valid_moves_without_children_assigned() {
        // Arrange
        let board = Board::setup(558380617816, 4362610851, [2,1,0,1,6,5,4]);
        let root = Some(ArcNode::new(NodeContent::new_root(board)));

        for i in 0..WIDTH {
            if i == 4 {
                continue;
            }
            insert_to_node_index(&mut root.clone().unwrap().clone(), i, board.play_move(i));
        }
        // Assert
        assert!(!root.unwrap().is_leaf())
    }
}
