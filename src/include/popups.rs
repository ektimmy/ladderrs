use derive_setters::Setters;
use ratatui::{
    prelude::{
        Buffer,
        Rect,
    },
    style::Style,
    text::{
        Line,
        Text,
    },
    widgets::{
        Block,
        Borders,
        Clear,
        Paragraph,
        Widget, 
    },
};

#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

