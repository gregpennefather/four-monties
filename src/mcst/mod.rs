use std::sync::Arc;

use crate::board::Board;

use self::node::{ArcNode, Link, Node, NodeContent};

pub mod node;

pub struct SearchTree {
    pub root: Link,
}

impl SearchTree {
    pub fn new(board: Board) -> Self {
        Self {
            root: Some(Arc::new(NodeContent::new_root(board))),
        }
    }

    pub fn expansion(&mut self, board: Board) {
        let root = self.root.as_ref().unwrap();
        let node = root.clone().seek(board);
        println!("expansion node: {:?}", node)
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
        assert!(tree.root.is_some());
    }
}
