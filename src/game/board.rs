use colored::Colorize;
use core::fmt::Debug;
use log::debug;

use super::result::GameResult;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;
pub const MAX_INDEX: usize = WIDTH * HEIGHT;

#[derive(Copy, Clone)]
pub struct Board {
    pub yellow_bb: u64,
    pub blue_bb: u64,
    pub column_pieces: [usize; WIDTH],
    pub yellow_turn: bool,
    pub result: Option<GameResult>,
    pub turn: u32
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.yellow_bb == other.yellow_bb && self.blue_bb == other.blue_bb
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            yellow_bb: 0,
            blue_bb: 0,
            column_pieces: [0; WIDTH],
            yellow_turn: true,
            result: None,
            turn: 0
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Board")
            .field(&self.yellow_bb)
            .field(&self.blue_bb)
            .field(&self.result)
            .finish()
    }
}

impl Board {
    pub fn get_moves(self) -> Vec<usize> {
        let mut available_moves: Vec<usize> = vec![];

        for column in 0..WIDTH {
            if self.column_pieces[column] != 6 {
                available_moves.push(column);
            }
        }

        available_moves
    }

    fn cell_empty(self, x: usize, y: usize) -> bool {
        self.column_pieces[x] < y
    }

    pub fn play_move(self, column: usize) -> Board {
        let mut n_b = self.clone();
        let row = self.column_pieces[column];
        let index = row * WIDTH + column; // Double check this
        if index >= MAX_INDEX {
            println!("attempting to play move: {column} in state {self:?} for {}", if self.yellow_turn { "Yellow" } else { "Blue" });
            self.print_board();
            println!("{:?}", self.get_moves());
            println!("{:?}", self.column_pieces);
            panic!("invalid move {column}");
        }
        let is_yellow = self.yellow_turn;
        if self.yellow_turn {
            n_b.yellow_bb ^= 1 << index;
        } else {
            n_b.blue_bb ^= 1 << index;
        }
        n_b.column_pieces[column] += 1;
        n_b.yellow_turn = !self.yellow_turn;
        n_b.turn += 1;
        n_b.update_winner(index, is_yellow);
        n_b
    }

    fn update_winner(&mut self, index: usize, yellow_player: bool) {
        let bb = if yellow_player {
            self.yellow_bb
        } else {
            self.blue_bb
        };

        debug!(
            "checking winner {} : v{}/h{}/d{}",
            if yellow_player {
                "Yellow".yellow()
            } else {
                "Blue".blue()
            },
            check_vertical(bb, index),
            check_horizontal(bb, index),
            check_diagonals(bb, index)
        );
        if bb.count_ones() > 3
            && (check_vertical(bb, index)
                || check_horizontal(bb, index)
                || check_diagonals(bb, index))
        {
            self.result = Some(match yellow_player {
                true => GameResult::YellowWin,
                false => GameResult::BlueWin,
            })
        }

        if self.result.is_none() && self.get_moves().len() == 0 {
            self.result = Some(GameResult::Draw)
        }
    }

    fn get_rank_str(&self, rank: usize) -> String {
        let mut str = String::default();
        let inverted_rank = HEIGHT - 1 - rank;
        for i in 0..WIDTH {
            let inverted_file = WIDTH - 1 - i;
            if self.blue_bb >> (inverted_file + inverted_rank * WIDTH) & 0b1 > 0 {
                str = format!("{}{}", str, &"0".blue());
            } else if self.yellow_bb >> (inverted_file + inverted_rank * WIDTH) & 0b1 > 0 {
                str = format!("{}{}", str, &"0".yellow());
            } else {
                str = format!("{}{}", str, &"X".dimmed());
            }
        }
        str
    }

    pub fn print_board(&self) {
        for rank in 0..HEIGHT {
            println!("{}", self.get_rank_str(rank));
        }
    }

    pub fn setup(yellow_bb: u64, blue_bb: u64, column_pieces: [usize; WIDTH]) -> Self {
        Self {
            yellow_bb: yellow_bb,
            blue_bb: blue_bb,
            yellow_turn: blue_bb.count_ones() == yellow_bb.count_ones(),
            column_pieces,
            result: None,
            turn: yellow_bb.count_ones() + blue_bb.count_ones() + 1
        }
    }
}

fn format_bb(bb: u64) -> String {
    let mut r: String = "".to_string();

    for i in 0..HEIGHT {
        let rank = HEIGHT - 1 - i;
        r += &format!("{:#09b}\n", (bb >> (rank * WIDTH) & 127));
    }

    r
}

fn check_diagonals(bb: u64, index: usize) -> bool {
    let mut start_pos = index;
    let rank: usize = index / WIDTH;
    for step_tl_br in 1..rank + 1 {
        let offset = step_tl_br * WIDTH + step_tl_br;
        if index < offset {
            break;
        }
        let pos = index - offset;

        if pos / WIDTH != rank - step_tl_br {
            break;
        }
        if bb >> pos & 1 == 1 {
            start_pos = pos;
        } else {
            break;
        }
    }
    if start_pos % WIDTH <= 3 {
        let relevant_bb = bb >> start_pos;
        debug!(
            "TL_BR: \t Starting pos {start_pos}\n{}",
            format_bb(relevant_bb)
        );

        if relevant_bb & 0x1010101 == 0x1010101 {
            return true;
        }
    } else {
        debug!("TL_BR: \t Skipping due to wrapping {start_pos}")
    }

    for step_bl_tr in 1..rank + 1 {
        let pos = index - (step_bl_tr * WIDTH - step_bl_tr);

        if pos / WIDTH != rank - step_bl_tr {
            break;
        }
        if bb >> pos & 1 == 1 {
            start_pos = pos;
        } else {
            break;
        }
    }

    if start_pos % WIDTH > 3 {
        let relevant_bb = if start_pos < 6 {
            bb << (7 - start_pos)
        } else {
            bb >> (start_pos - 6)
        };
        relevant_bb & 0x1041040 == 0x1041040
    } else {
        debug!("BL_TR: \t Skipping due to wrapping {start_pos}");
        false
    }
}

fn check_horizontal(mut bb: u64, index: usize) -> bool {
    bb = bb & (0x7F << WIDTH * (index / WIDTH));
    let mut start_pos = index;
    let horizontal_position = index % WIDTH;

    for right_index in 1..horizontal_position + 1 {
        let check_pos = index - right_index;
        if bb >> check_pos & 1 == 0 {
            break;
        } else {
            start_pos = check_pos;
        }
    }

    let relevant_bb = bb >> start_pos;
    relevant_bb & 0xF == 0xF
}

fn check_vertical(bb: u64, index: usize) -> bool {
    if index < 3 * WIDTH {
        return false;
    }
    bb >> (index - (WIDTH * 3)) & 0x204081 == 0x204081
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn check_vertical_valid_win() {
        let file = 4;
        let bb = 0x204081 << file;
        let board = Board::setup(bb, 0, [0; WIDTH]);

        board.print_board();
        assert!(check_vertical(bb, (file + WIDTH * 3)))
    }

    #[test]
    pub fn check_vertical_below_4th_row_fails() {
        let file = 6;
        let bb = 0x4081 << file;
        let board = Board::setup(bb, 0, [0; WIDTH]);

        board.print_board();
        assert!(!check_vertical(bb, file + WIDTH * 2))
    }

    #[test]
    pub fn check_vertical_above_4th_row_but_missing_a_position() {
        let file = 5;
        let bb = 0x10200080 << file;
        let board = Board::setup(bb, 0, [0; WIDTH]);

        board.print_board();
        assert!(!check_vertical(bb, file + WIDTH * 4))
    }

    #[test]
    pub fn check_vertical_win_with_noise() {
        let file = 5;
        let bb = 0x14606188 << file;
        let board = Board::setup(bb, 0, [0; WIDTH]);

        board.print_board();
        assert!(check_vertical(bb, file + WIDTH * 4))
    }

    #[test]
    pub fn check_horizontal_valid_win() {
        let bb = 0x78;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(check_horizontal(bb, 5));
        assert!(check_horizontal(bb, 6));
        assert!(check_horizontal(bb, 4));
        assert!(check_horizontal(bb, 3))
    }

    #[test]
    pub fn check_horizontal_no_wrapping_wins() {
        let bb = 0xF0;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(!check_horizontal(bb, 7));
        assert!(!check_horizontal(bb, 6));
        assert!(!check_horizontal(bb, 5));
        assert!(!check_horizontal(bb, 4))
    }

    #[test]
    pub fn check_horizontal_only_win_on_left_side() {
        let bb = 0x1EC000;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(!check_horizontal(bb, 14));
        assert!(check_horizontal(bb, 18));
    }

    #[test]
    pub fn check_horizontal_case_0() {
        let bb = 0x2020F65;

        let board = Board::setup(bb, 0, [0; WIDTH]);
        board.print_board();
        assert!(check_horizontal(bb, 8));
    }

    #[test]
    pub fn check_horizontal_case_1() {
        let bb = 0x8F;

        let board = Board::setup(bb, 0, [0; WIDTH]);
        board.print_board();
        assert!(check_horizontal(bb, 1));
    }

    #[test]
    pub fn check_diagonal_valid_starting_in_bottom_corner_0() {
        let bb = 0x1010101;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(check_diagonals(bb, 0));
    }

    #[test]
    pub fn check_diagonal_valid_starting_in_bottom_corner_6() {
        let bb = 0x1041040;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(check_diagonals(bb, 24));
    }

    #[test]
    pub fn check_diagonal_bl_not_on_file_0() {
        let bb = 0x1041040 << 6;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(check_diagonals(bb, 30));
    }

    #[test]
    pub fn check_diagonal_br_not_on_file_0() {
        let bb = 0x1010101 << 10;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(check_diagonals(bb, 26));
    }

    #[test]
    pub fn check_diagonal_bl_wrapping_fails() {
        let bb = 0x1041040 >> 5;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(!check_diagonals(bb, 13));
    }

    #[test]
    pub fn check_diagonal_br_wrapping_fails() {
        let bb = 0x1010101 << 12;
        let board = Board::setup(0, bb, [0; WIDTH]);

        board.print_board();
        assert!(!check_diagonals(bb, 28));
    }

    #[test]
    pub fn check_diagonal_case_0() {
        let bb = 0x8219;

        let board = Board::setup(bb, 0, [0; WIDTH]);
        board.print_board();
        assert!(!check_diagonals(bb, 9));
    }

    #[test]
    pub fn check_diagonal_case_1() {
        let bb = 0x10099;

        let board = Board::setup(0, bb, [0; WIDTH]);
        board.print_board();
        assert!(!check_diagonals(bb, 7));
    }

    #[test]
    pub fn check_diagonal_case_2() {
        let bb = 13314539663852;

        let board = Board::setup(bb, 0, [0; WIDTH]);
        board.print_board();
        assert!(!check_diagonals(bb, 19));
    }

    #[test]
    pub fn check_diagonal_case_3() {
        let bb = 0x104104;

        let board = Board::setup(bb, 0, [0; WIDTH]);
        board.print_board();
        assert!(!check_diagonals(bb, 3));
    }
    #[test]
    pub fn check_diagonal_case_4() {
        let bb = 0x208208;

        let board = Board::setup(bb, 0, [0; WIDTH]);
        board.print_board();
        assert!(check_diagonals(bb, 4));
    }

    #[test]
    pub fn update_winner_move_leading_to_draw() {
        // Arrange
        let b = Board::setup(890452430364, 1308570825187, [6, 6, 6, 6, 6, 6, 5]);

        // Act
        let r = b.play_move(6);

        // Assert
        assert_eq!(r.result, Some(GameResult::Draw));
    }
}
