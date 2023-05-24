use std::time::{Duration, Instant};

use crossterm::event::{Event, self, KeyCode};
use tui::{backend::Backend, Terminal, layout::{Layout, Direction, Constraint, Alignment}};

use self::{utils::draw_block_with_text, input::{UserInput, Target}, pomo_ui::{starter::PomodoroStarterUI, clock::PomodoroClockUI}, ui::UI};

mod pomo_ui {
    pub mod starter;
    pub mod clock;
}

mod utils;
mod input;
mod ui;

pub struct App<B: Backend> {
    terminal: Terminal<B>,
}

struct AppContext<B: Backend> {
    stack: Vec<Box<dyn UI<B>>>,
}

impl<B: Backend> AppContext<B> {
    pub fn last(&mut self) -> &mut dyn UI<B> {
        self.stack.last_mut().unwrap().as_mut()
    }

    pub fn push<T: UI<B> + 'static>(&mut self, ui: T) {
        self.stack.push(Box::new(ui))
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}


impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }
    
    /// Main function to run the application
    pub fn run(&mut self, tick_rate: Duration) {
        let mut input = UserInput::None;
        let mut ctx = AppContext { stack: vec![Box::new(PomodoroStarterUI::new())] };

        // Burner read to remove any buffered input
        event::read().unwrap();

        let mut last_tick = Instant::now();
        //let mut skip_this = false;
        loop {
            self.terminal.draw(|f| Self::ui(&mut ctx, f, &mut input)).unwrap();

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            if let UserInput::Goto(target) = &input {
                match target {
                    Target::Pomodoro => ctx.push(PomodoroClockUI::new()),
                    Target::PopStack => ctx.pop(),
                    Target::Quit => return,
                }
            }

            if input.is_consumed() {
                input = UserInput::None;
                event::read().unwrap();
                continue;
            }

            let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(Duration::from_secs(0));
            if !event::poll(timeout).unwrap() {
                continue;
            }

            if let Event::Key(key) = event::read().unwrap() {
                input = match key.code {
                    KeyCode::Up => UserInput::Up,
                    KeyCode::Down => UserInput::Down,
                    KeyCode::Left => UserInput::Left,
                    KeyCode::Right => UserInput::Right,
                    KeyCode::Enter => UserInput::Enter,
                    _ => UserInput::None
                }
            }
        }
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal<B> {
        &mut self.terminal
    }

    /// Main function to draw ui
    fn ui(ctx: &mut AppContext<B>, frame: &mut tui::Frame<B>, input: &mut UserInput) {
        let layout = Layout::default().margin(0).direction(Direction::Vertical).constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ]).split(frame.size());

        draw_block_with_text(" Overfocus | Pomodoro ", Alignment::Center, frame, layout[0]);
        ctx.last().ui(frame, layout[1], input);
        draw_block_with_text(" [00:00:00] Pomodoro started!", Alignment::Left, frame, layout[2]);
    }

    
}