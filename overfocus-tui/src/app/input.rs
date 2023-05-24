#[derive(PartialEq, Eq)]
pub enum UserInput {
    None,
    Up, Right, Left, Down,
    Enter,

    /// Defines a redirection to another part of the ui
    Goto(Target),
    Consumed,
}

#[derive(PartialEq, Eq)]
pub enum Target {
    Pomodoro,
    PopStack,
    Quit,
}

impl UserInput {
    pub fn consume_matches<T>(&mut self, pattern: impl FnOnce(&Self) -> bool, ev: impl FnOnce(&mut Self) -> T) -> Option<T> {
        if pattern(self) {
            *self = UserInput::Consumed;
            return Some(ev(self));
        }
        None
    }

    pub fn is_consumed(&self) -> bool {
        matches!(self, UserInput::Goto(_) | UserInput::Consumed)
    }
}