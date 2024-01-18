#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameResult {
    YellowWin,
    Draw,
    BlueWin,
}

impl GameResult {
    pub fn fair_result(&self) -> Self {
        match self {
            GameResult::YellowWin => GameResult::YellowWin,
            GameResult::BlueWin => GameResult::BlueWin,
            GameResult::Draw => match rand::random() {
                true => GameResult::YellowWin,
                false => GameResult::BlueWin,
            },
        }
    }
}
