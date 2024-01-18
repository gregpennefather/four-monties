
#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub enum ValidMove<T> {
    Valid(T),
    Invalid
}

impl<T> ValidMove<T> {
    pub fn is_valid(&self) -> bool {
        matches!(*self, ValidMove::Valid(_))
    }
}