use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::ui::helper;

pub fn render(frame: &mut Frame<impl tui::backend::Backend>, term: String) {
    let size = frame.size();
    let block = Block::default().title("Search").borders(Borders::ALL);
    let area = helper::centered_rect(60, 60, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(block.inner(area));
    let search = Paragraph::new(format!("{}", term))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    frame.render_widget(Clear, area);
    frame.render_widget(search, chunks[0]);
    frame.render_widget(block, area);
}
