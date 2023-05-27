use tui::{widgets::{Block, Paragraph, Borders}, layout::{Alignment, Rect}, backend::Backend, text::Text};

use super::styles::regular_style;

pub fn draw_block_with_text<'a, B: Backend, T: Into<Text<'a>>>(text: T, alignment: Alignment, frame: &'a mut tui::Frame<B>, rect: Rect) {
    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(text).alignment(alignment).block(block).style(regular_style());
    frame.render_widget(paragraph, rect);
}

/// Returs a Rect centered in the source rect with a given size
pub fn sub_rect(rect: Rect, size: (u16, u16)) -> Rect {
    Rect::new(
        rect.x + (rect.width / 2) - (size.0 / 2),
        rect.y + (rect.height / 2) - (size.1 / 2),
        size.0,
        size.1
    )
}