use tui::{backend::Backend, layout::{Rect, Alignment}, style::{Color, Style, Modifier}, text::{Span, Spans}, widgets::{Block, Borders, Paragraph}};

use crate::app::{utils::sub_rect, input::UserInput};

struct Stats { max: u8, cur: u8, avg: u8 }

#[allow(unused)]
pub struct PomodoroStarterUI {
    stats: Stats,
    selected: u8,
}

impl PomodoroStarterUI {
    pub fn new() -> Self {
        Self { stats: Stats { max: 0, cur: 0, avg: 0 }, selected: 0 }
    }
    
    pub fn ui<B: Backend>(&mut self, frame: &mut tui::Frame<B>, rect: Rect, input: &mut UserInput) {
        // Handle input
        input.consume_eq(UserInput::Up, |_| if self.selected == 1 { self.selected = 0 });
        input.consume_eq(UserInput::Down, |_| if self.selected == 0 { self.selected = 1 });
        if input.consume_eq(UserInput::Enter, |input| {
            if self.selected == 0 {
                todo!();
            } else {
                *input = UserInput::Quit // Convert user input to quit so the application quits
            }
        }).is_some() { return }

        // Create layout
        let rect = sub_rect(rect, (20, 8));

        // Actually do shit
        let text = self.get_spans();

        let block = Block::default().borders(Borders::ALL).title(" [ Pomodoro ] ").title_alignment(Alignment::Center);
        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, rect);
    }

    fn get_spans(&self) -> Vec<Spans> {
        let mut res = vec![
            Spans::from(format!("Max: {}", self.stats.max)),
            Spans::from(format!("Cur: {}", self.stats.cur)),
            Spans::from(format!("Avg: {}", self.stats.avg)),
            Spans::from(""),
        ];

        if self.selected == 0 {
            res.extend([
                Spans::from(Span::styled(">Start", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
                Spans::from(Span::styled("-Exit", Style::default().fg(Color::White))),
            ]);
        } else {
            res.extend([
                Spans::from(Span::styled("-Start", Style::default().fg(Color::White))),
                Spans::from(Span::styled(">Exit", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
            ]);
        }
        res
    }
}