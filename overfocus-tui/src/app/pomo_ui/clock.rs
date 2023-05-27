use overfocus::pomodoro::{PomodoroHandle, Pomodoro, PomodoroStage};
use tui::{backend::Backend, text::{Spans, Span}, widgets::{Block, Borders, Paragraph}, layout::Alignment};

use crate::app::{ui::UI, utils::sub_rect, input::{UserInput, Target}, styles::{regular_style, highlight_style}};

pub struct PomodoroClockUI {
    clock: PomodoroHandle,
    selected: u8,
    paused: bool,
}

impl<B: Backend> UI<B> for PomodoroClockUI {
    fn ui(&mut self, frame: &mut tui::Frame<B>, rect: tui::layout::Rect, input: &mut UserInput) {
        // Handle Events
        input.consume_matches(|x| matches!(x, UserInput::Up), |_| if self.selected == 1 { self.selected = 0 });
        input.consume_matches(|x| matches!(x, UserInput::Down), |_| if self.selected == 0 { self.selected = 1 });
        if input.consume_matches(|x| matches!(x, UserInput::Enter), |input| {
            if self.selected == 1 {
                *input = UserInput::Goto(Target::PopStack);
                Pomodoro::stop(&self.clock).unwrap()
            } else {
                if self.paused {
                    Pomodoro::resume(&self.clock).unwrap()
                } else {
                    Pomodoro::pause(&self.clock).unwrap()
                }
                self.paused = !self.paused;
            }
        }).is_some() { return }
        
        // Display things
        let rect = sub_rect(rect, (20, 8));

        let block = Block::default().borders(Borders::ALL).title(" [ Pomodoro ] ").title_alignment(Alignment::Center).style(regular_style());
        let paragraph = Paragraph::new(self.get_spans()).block(block).style(regular_style());
        frame.render_widget(paragraph, rect);
    }
}

impl PomodoroClockUI {
    pub fn new() -> Self {
        let clock = Pomodoro::start();
        Self { clock, selected: 0, paused: false }
    }

    fn get_spans(&self) -> Vec<Spans> {
        let (secs, reps, pomos, stage) = Pomodoro::lock_and(&self.clock, |x| (x.seconds(), x.repetitions(), x.pomodoros(), *x.stage())).unwrap();
        
        let mut res = vec![
            Spans::from(format!("Pomodoros: {}", pomos)),
            Spans::from(format!("Stage: {}", Self::display_stage(stage, reps))),
            Spans::from(format!("Elapsed: ({}:{:02})", secs / 60, secs % 60)),
            Spans::from(""),
        ];

        if self.selected == 0 {
            res.extend([
                Spans::from(Span::styled(format!(">{}", self.get_pause_text()), highlight_style())),
                Spans::from(Span::styled("-Stop and exit", regular_style())),
            ]);
        } else {
            res.extend([
                Spans::from(Span::styled(format!("-{}", self.get_pause_text()), regular_style())),
                Spans::from(Span::styled(">Stop and exit", highlight_style())),
            ]);
        }
        res
    }

    fn get_pause_text(&self) -> &str {
        if self.paused { "Resume" } else { "Pause" }
    }

    fn display_stage(stage: PomodoroStage, reps: u8) -> String {
        match stage {
            PomodoroStage::Work => format!("Work ({}/3)", reps + 1),
            PomodoroStage::ShortBreak => format!("Break ({}/3)", reps + 1),
            PomodoroStage::LongBreak => "Long Break".to_string(),
        }
    }
}