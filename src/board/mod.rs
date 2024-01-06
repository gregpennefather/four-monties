use array2d::Array2D;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

type CellValue = Option<bool>;

#[derive(Copy, Clone)]
pub struct Board {
    pub yellow_bb: u64,
    pub blue_bb: u64,
    pub column_pieces: [usize; WIDTH],
    pub yellow_turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            yellow_bb: 0,
            blue_bb: 0,
            column_pieces: [0; WIDTH],
            yellow_turn: true,
        }
    }
}

impl Board {
    pub fn get_moves(self) -> Vec<usize> {
        let mut available_moves: Vec<usize> = vec![];

        for i in 0..WIDTH {
            available_moves.push(i);
        }

        available_moves
    }

    fn cell_empty(self, x: usize, y: usize) -> bool {
        self.column_pieces[x] < y
    }

    pub fn play_move(self, column: usize) -> Board {
        let mut n_b = self.clone();
        let row = self.column_pieces[column];
        if self.yellow_turn {
            n_b.yellow_bb ^= 1 << row*WIDTH + column; // Double check this
        } else {
            n_b.blue_bb ^= 1 << row*WIDTH + column; // Double check this
        }
        n_b.column_pieces[column] += 1;
        n_b.yellow_turn = !self.yellow_turn;
        n_b
    }
}
