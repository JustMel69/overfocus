use tui::{backend::Backend, layout::{Rect, Alignment}, style::{Color, Style, Modifier}, text::{Span, Spans}, widgets::{Block, Borders, Paragraph}};

use crate::app::{utils::sub_rect, input::{UserInput, Target}, ui::UI};

struct Stats { max: u8, cur: u8, avg: u8 }

/// The struct that holds the information of the pomodoro starting screen
pub struct PomodoroStarterUI {
    stats: Stats,
    selected: u8,
}

impl<B: Backend> UI<B> for PomodoroStarterUI {
    fn ui(&mut self, frame: &mut tui::Frame<B>, rect: Rect, input: &mut UserInput) {
        // Handle input
        input.consume_matches(|x| matches!(x, UserInput::Up), |_| if self.selected == 1 { self.selected = 0 });
        input.consume_matches(|x| matches!(x, UserInput::Down), |_| if self.selected == 0 { self.selected = 1 });
        if input.consume_matches(|x| matches!(x, UserInput::Enter), |input| {
            // Converts input to redirections
            *input = UserInput::Goto(if self.selected == 0 { Target::Pomodoro } else { Target::Quit })
        }).is_some() { return }

        // Create layout
        let rect = sub_rect(rect, (20, 8));

        // Actually do shit
        let text = self.get_spans();

        let block = Block::default().borders(Borders::ALL).title(" [ Pomodoro ] ").title_alignment(Alignment::Center);
        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, rect);
    }
}

impl PomodoroStarterUI {
    pub fn new() -> Self {
        Self { stats: Stats { max: 0, cur: 0, avg: 0 }, selected: 0 }
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