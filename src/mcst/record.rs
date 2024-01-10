use super::playout::PlayoutResult;

#[derive(Clone, Copy, Debug, Default)]
pub struct Record {
    wins: u16,
    played: u16,
}

impl Record {
    pub fn increment(&mut self, result: PlayoutResult) {
        self.played += 1;
        self.wins += match result {
            PlayoutResult::Win => 1,
            _ => 0,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}/{}", self.wins, self.played).as_str())
    }
}
