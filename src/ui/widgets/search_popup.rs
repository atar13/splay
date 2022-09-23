use tui::{Frame, widgets::{Block, Borders, Paragraph, Clear, Wrap}, style::{Style, Color}, layout::Alignment, text::{Text, Span}};

use crate::ui::helper;


pub fn render(frame: &mut Frame<impl tui::backend::Backend>, term: String) {
    let size = frame.size();
    let block = Block::default().title("Search").borders(Borders::ALL);
    let area = helper::centered_rect(60, 60, size);
    let search = Paragraph::new(format!("{}", term))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    frame.render_widget(Clear, area);
    frame.render_widget(search, block.inner(area));
    frame.render_widget(block, area);
}
