use tui::style::{Style, Color};

pub fn regular_style() -> Style   { Style::default().fg(Color::White) }
pub fn highlight_style() -> Style { Style::default().fg(Color::Yellow) }
pub fn info_log_style() -> Style  { Style::default().fg(Color::White) }
pub fn warn_log_style() -> Style  { Style::default().fg(Color::Yellow) }
pub fn err_log_style() -> Style   { Style::default().fg(Color::Red) }