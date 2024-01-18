use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Yellow,
    Blue,
    NoPlayer,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Player {
    pub fn invert(&self) -> Self {
        match self {
            Player::Yellow => Player::Blue,
            Player::Blue => Player::Yellow,
            Player::NoPlayer => match rand::random() {
                true => Player::Yellow,
                false => Player::Blue,
            },
        }
    }
}