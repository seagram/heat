use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, _app: &App) {
    let area = frame.area();

    let layout = Layout::vertical([Constraint::Min(0)]).split(area);

    let text = Text::raw("heat").centered();
    let paragraph = Paragraph::new(text).style(Style::default());
    frame.render_widget(paragraph, layout[0]);
}
