use tui::{backend::Backend, Terminal};

pub struct App<B: Backend> {
    terminal: Terminal<B>,
}


impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }
    
    pub fn run(&mut self) {
        loop {
            self.terminal.draw(|f| Self::ui(f)).unwrap();
        }
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal<B> {
        &mut self.terminal
    }

    fn ui(frame: &mut tui::Frame<B>) {
        todo!()

    }
}