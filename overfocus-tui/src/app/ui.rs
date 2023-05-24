use tui::{backend::Backend, layout::Rect};

use super::input::UserInput;

pub trait UI<B: Backend> {
    fn ui(&mut self, frame: &mut tui::Frame<B>, rect: Rect, input: &mut UserInput);
}