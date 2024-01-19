use crate::game::result::GameResult;

#[derive(Clone, Copy, Debug, Default)]
pub struct Record {
    pub wins: u64,
    pub played: u64,
}

impl Record {
    pub fn increment(&mut self, win: bool) {
        self.played += 1;
        self.wins += match win {
            true => 1,
            _ => 0,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}/{}", self.wins, self.played).as_str())
    }
}
