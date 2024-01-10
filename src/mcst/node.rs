use std::sync::{Arc, Mutex, RwLock, Weak};

use crate::board::{Board, WIDTH};

use super::{playout::PlayoutResult, record::Record};

#[derive(Debug)]
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

pub fn insert_to_node_index(root: &mut ArcNode, index: usize, board: Board) -> &mut Arc<NodeContent> {
    let child = Some(root.new_child(index, board));
    match root.children.try_write() {
        Ok(mut children) => children[index] = child,
        Err(e) => panic!("Err {e:?}!"),
    }
    root
}

impl Node for ArcNode {

    fn board(&self) -> Board {
        self.board
    }

    fn record_result(&mut self, result: PlayoutResult) {
        match self.record.try_write() {
            Ok(mut r) => r.increment(result),
            Err(e) => panic!("Record result lock error {e:?}")
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
        let board1 = Board::setup(1, 0);
        let board2 = Board::setup(2, 0);
        let root = Some(ArcNode::new(NodeContent::new_root(Board::default())));

        insert_to_node_index(&mut root.clone().unwrap().clone(), 0, board1);
        insert_to_node_index(&mut root.clone().unwrap().clone(), 1, board2);

        // Act
        let seek = root.unwrap().seek(board2);

        // Assert
        assert!(seek.is_some());
        assert_eq!(seek.as_ref().unwrap().board, board2)
    }
}
