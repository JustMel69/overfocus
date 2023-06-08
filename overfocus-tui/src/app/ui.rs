use tui::{backend::Backend, layout::Rect};

use super::input::UserInput;

pub trait UI<B: Backend> {
    fn ui(&mut self, frame: &mut tui::Frame<B>, rect: Rect, input: &mut UserInput);
    fn handle_context(&mut self, _ctx: UIContext) { }
    fn get_context(&self) -> Option<UIContext> { None }
}

pub enum UIContext {
    PomodoroClock { pomodoros: u8 }
}