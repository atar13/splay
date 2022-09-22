use tui::{Frame, widgets::{Block, Borders, Paragraph, Clear}, style::{Style, Color}, layout::Alignment};

use crate::ui::helper;


pub fn render(frame: &mut Frame<impl tui::backend::Backend> ) {
    let size = frame.size();
    let block = Block::default().title("Popup").borders(Borders::ALL);
    let area = helper::centered_rect(60, 60, size);
    let paragraph = Paragraph::new("searching stuff")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, block.inner(area));
    frame.render_widget(block, area);
}
