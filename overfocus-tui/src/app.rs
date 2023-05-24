use crossterm::event::{Event, self, KeyCode};
use tui::{backend::Backend, Terminal, layout::{Layout, Direction, Constraint, Alignment}};

use self::{utils::draw_block_with_text, input::UserInput, pomo_ui::starter::PomodoroStarterUI};

mod pomo_ui {
    pub mod starter;
}

mod utils;
mod input;

pub struct App<B: Backend> {
    terminal: Terminal<B>,
}

struct AppContext {
    starter: PomodoroStarterUI,
}

impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }
    
    pub fn run(&mut self) {
        let mut input = UserInput::None;
        let mut ctx = AppContext { starter: PomodoroStarterUI::new() };

        loop {
            self.terminal.draw(|f| Self::ui(&mut ctx, f, &mut input)).unwrap();

            if matches!(input, UserInput::Quit) {
                return;
            }

            if let Event::Key(key) = event::read().unwrap() {
                input = match key.code {
                    KeyCode::Up => UserInput::Up,
                    KeyCode::Down => UserInput::Down,
                    KeyCode::Left => UserInput::Left,
                    KeyCode::Right => UserInput::Right,
                    KeyCode::Tab => UserInput::Enter,
                    _ => UserInput::None
                }
            }
        }
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal<B> {
        &mut self.terminal
    }

    fn ui(ctx: &mut AppContext, frame: &mut tui::Frame<B>, input: &mut UserInput) {
        let layout = Layout::default().margin(0).direction(Direction::Vertical).constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ]).split(frame.size());

        draw_block_with_text(" Overfocus | Pomodoro ", Alignment::Center, frame, layout[0]);
        ctx.starter.ui(frame, layout[1], input);
        draw_block_with_text(" [00:00:00] Pomodoro started!", Alignment::Left, frame, layout[2]);
    }

    
}