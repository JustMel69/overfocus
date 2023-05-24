#[derive(PartialEq, Eq)]
pub enum UserInput {
    None,
    Up, Right, Left, Down,
    Enter,
    Quit,
}

impl UserInput {
    pub fn consume_eq<T>(&mut self, pattern: UserInput, ev: impl FnOnce(&mut Self) -> T) -> Option<T> {
        if *self == pattern {
            *self = UserInput::None;
            return Some(ev(self));
        }
        None
    }
}