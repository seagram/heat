use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, _app: &App) {
    let area = frame.area();

    let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);

    let main_area = layout[0];
    let footer_area = layout[1];

    // Main area - placeholder for now
    let placeholder = Paragraph::new("heat").centered();
    frame.render_widget(placeholder, main_area);

    // Controls bar footer
    let controls = render_controls_bar();
    frame.render_widget(controls, footer_area);
}

fn render_controls_bar() -> Paragraph<'static> {
    let separator = Span::styled(" │ ", Style::default().fg(Color::DarkGray));

    let controls = Line::from(vec![
        Span::styled("j/k", Style::default().fg(Color::Yellow)),
        Span::raw(": navigate"),
        separator.clone(),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": toggle today"),
        separator.clone(),
        Span::styled("a", Style::default().fg(Color::Yellow)),
        Span::raw(": add"),
        separator.clone(),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(": rename"),
        separator.clone(),
        Span::styled("D", Style::default().fg(Color::Yellow)),
        Span::raw(": delete"),
        separator,
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(": quit"),
    ]);

    Paragraph::new(controls).centered()
}
