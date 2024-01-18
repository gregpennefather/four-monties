use crate::{game::board::Board, mcst::SearchTree};

#[test]
pub fn winning_move_possible() {
    // Act
    let mut tree = SearchTree::new(Board::setup(7, 112, [1, 1, 1, 0, 1, 1, 1]));

    // Act
    for i in 0..10 {
        tree.iterate();
    }

    let m = tree.select_move();

    // Assert
    assert_eq!(m, 3);
}

#[test]
pub fn opponent_can_win_next_move_should_block() {
    // Act
    let b = Board::setup(7, 96, [1, 1, 0, 0, 1, 1, 1]);
    let mut tree = SearchTree::new(b);

    // Act
    for i in 0..200 {
        tree.iterate();
    }

    tree.print_state();
    b.print_board();

    let m = tree.select_move();

    // Assert
    assert_eq!(m, 3);
}

#[test]
pub fn opponent_can_win_next_move_should_but_so_can_player_should_win() {
    // Act
    let b = Board::setup(16513, 14, [3, 1, 1, 1, 0, 0, 0]);
    let mut tree = SearchTree::new(b);

    // Act
    for i in 0..200 {
        tree.iterate();
    }

    tree.print_state();
    b.print_board();

    let m = tree.select_move();

    // Assert
    assert_eq!(m, 0);
}
