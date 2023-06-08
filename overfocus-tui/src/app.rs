use std::time::{Duration, Instant};

use crossterm::event::{Event, self, KeyCode};
use overfocus::logger::{Logger, LogKind, self};
use tui::{backend::Backend, Terminal, layout::{Layout, Direction, Constraint, Alignment, Rect}, widgets::{Block, Borders, Paragraph}};

use self::{utils::draw_block_with_text, input::{UserInput, Target}, pomo_ui::{starter::PomodoroStarterUI, clock::PomodoroClockUI}, ui::{UI, UIContext}, styles::{info_log_style, warn_log_style, err_log_style, regular_style}, notifications::notify};

mod pomo_ui {
    pub mod starter;
    pub mod clock;
}

mod utils;
mod input;
mod ui;
mod styles;
mod notifications;

pub struct App<B: Backend> {
    terminal: Terminal<B>,
}

struct AppContext<B: Backend> {
    stack: Vec<Box<dyn UI<B>>>,
}

impl<B: Backend> AppContext<B> {
    pub fn peek(&mut self) -> &mut dyn UI<B> {
        self.stack.last_mut().unwrap().as_mut()
    }

    pub fn push<T: UI<B> + 'static>(&mut self, ui: T) {
        self.stack.push(Box::new(ui))
    }

    pub fn pop(&mut self) -> Option<UIContext> {
        self.stack.pop().map(|x| x.get_context()).flatten()
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

        Logger::init();

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
                    Target::PopStack => {
                        if let Some(data) = ctx.pop() {
                            ctx.peek().handle_context(data);
                        }
                    },
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
        ctx.peek().ui(frame, layout[1], input);
        Self::draw_logger(frame, layout[2]);
    }

    fn draw_logger(frame: &mut tui::Frame<B>, rect: Rect) {
        let last = Logger::last().map(|x| (x.0, x.1, x.2));
        let (str, kind) = match last {
            Some((txt, kind, secs)) => (format!(" [{:02}:{:02}:{:02}] {}", secs / 3600, (secs / 60) % 60, secs % 60, txt), kind),
            None => (String::new(), LogKind::Info),
        };

        let style = match kind {
            LogKind::Info => info_log_style(),
            LogKind::Warn => warn_log_style(),
            LogKind::Err => err_log_style(),
        };

        let block = Block::default().borders(Borders::ALL).style(regular_style());
        let paragraph = Paragraph::new(str).alignment(Alignment::Left).block(block).style(style);
        frame.render_widget(paragraph, rect);

        if let Some(x) = Logger::consume_notification() {
            notify("Overfocus", &x.0, matches!(x.1, logger::Duration::Long)).unwrap();
        }
    }   
}