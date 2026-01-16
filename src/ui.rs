use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::data::Habit;

const CARD_HEIGHT: u16 = 11;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);

    let main_area = layout[0];
    let footer_area = layout[1];

    // Main area
    if app.data.habits.is_empty() {
        let empty_state = render_empty_state();
        frame.render_widget(empty_state, main_area);
    } else {
        render_habit_list(frame, app, main_area);
    }

    // Controls bar footer
    let controls = render_controls_bar();
    frame.render_widget(controls, footer_area);
}

fn render_habit_list(frame: &mut Frame, app: &App, area: Rect) {
    let card_constraints: Vec<Constraint> = app
        .data
        .habits
        .iter()
        .map(|_| Constraint::Length(CARD_HEIGHT))
        .collect();

    let card_areas = Layout::vertical(card_constraints).split(area);

    for (i, habit) in app.data.habits.iter().enumerate() {
        if i < card_areas.len() {
            let is_selected = i == app.selected_index;
            render_habit_card(frame, habit, card_areas[i], is_selected);
        }
    }
}

fn render_habit_card(frame: &mut Frame, habit: &Habit, area: Rect, is_selected: bool) {
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!(" {} ", habit.name));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let content_layout = Layout::vertical([
        Constraint::Length(1), // Stats row
        Constraint::Length(1), // Empty line
        Constraint::Min(0),    // Heatmap area
    ])
    .split(inner_area);

    // Stats row placeholder
    let stats = Paragraph::new(Line::from(vec![
        Span::raw("🔥 "),
        Span::raw("-- day streak"),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("Best: --"),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("--% (3 mo)"),
    ]));
    frame.render_widget(stats, content_layout[0]);

    // Heatmap placeholder
    let day_labels = ["S", "M", "T", "W", "T", "F", "S"];
    let heatmap_lines: Vec<Line> = day_labels
        .iter()
        .map(|label| {
            Line::from(vec![
                Span::raw(format!("{} ", label)),
                Span::styled("□□□□□□□ □□□□□□□ □□□□□□□", Style::default().fg(Color::DarkGray)),
            ])
        })
        .collect();

    let heatmap = Paragraph::new(heatmap_lines);
    frame.render_widget(heatmap, content_layout[2]);
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

fn render_empty_state() -> Paragraph<'static> {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "No habits yet",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("a", Style::default().fg(Color::Yellow)),
            Span::raw(" to add your first habit"),
        ]),
    ];

    Paragraph::new(lines).centered()
}
