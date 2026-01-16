use chrono::{Datelike, Duration, NaiveDate, Weekday};
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

    // Stats row
    let current_streak = habit.current_streak();
    let longest_streak = habit.longest_streak();
    let completion_pct = habit.completion_percentage();

    let streak_text = if current_streak == 1 {
        "1 day streak".to_string()
    } else {
        format!("{} day streak", current_streak)
    };

    let stats = Paragraph::new(Line::from(vec![
        Span::raw("🔥 "),
        Span::raw(streak_text),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw(format!("Best: {}", longest_streak)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw(format!("{}% (3 mo)", completion_pct)),
    ]));
    frame.render_widget(stats, content_layout[0]);

    // Heatmap grid
    let heatmap_lines = build_heatmap(habit);
    let heatmap = Paragraph::new(heatmap_lines);
    frame.render_widget(heatmap, content_layout[2]);
}

fn build_heatmap(habit: &Habit) -> Vec<Line<'static>> {
    let today = chrono::Local::now().date_naive();
    let three_months_ago = today - Duration::days(90);

    // Find the Sunday at or before three_months_ago to start the grid
    let start_date = find_previous_sunday(three_months_ago);

    // Find the Saturday at or after today to end the grid
    let end_date = find_next_saturday(today);

    // Calculate number of weeks
    let num_weeks = ((end_date - start_date).num_days() / 7 + 1) as usize;

    // Build grid: 7 rows (Sun=0 through Sat=6), num_weeks columns
    let mut grid: Vec<Vec<Option<bool>>> = vec![vec![None; num_weeks]; 7];

    let mut current_date = start_date;
    for week in 0..num_weeks {
        for day in 0..7 {
            if current_date <= today {
                let is_completed = habit.completions.contains(&current_date);
                grid[day][week] = Some(is_completed);
            }
            current_date += Duration::days(1);
        }
    }

    // Build display lines
    let day_labels = ["S", "M", "T", "W", "T", "F", "S"];
    let mut lines: Vec<Line<'static>> = Vec::new();

    for (row_idx, label) in day_labels.iter().enumerate() {
        let mut spans: Vec<Span<'static>> = vec![Span::raw(format!("{} ", label))];

        for (week_idx, cell) in grid[row_idx].iter().enumerate() {
            let span = match cell {
                Some(true) => Span::styled("■", Style::default().fg(Color::Green)),
                Some(false) => Span::styled("□", Style::default().fg(Color::DarkGray)),
                None => Span::raw(" "), // Future date
            };
            spans.push(span);

            // Add space between weeks (every 7 days)
            if week_idx < grid[row_idx].len() - 1 {
                spans.push(Span::raw(" "));
            }
        }

        lines.push(Line::from(spans));
    }

    lines
}

fn find_previous_sunday(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    let days_since_sunday = match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    };
    date - Duration::days(days_since_sunday)
}

fn find_next_saturday(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    let days_until_saturday = match weekday {
        Weekday::Sun => 6,
        Weekday::Mon => 5,
        Weekday::Tue => 4,
        Weekday::Wed => 3,
        Weekday::Thu => 2,
        Weekday::Fri => 1,
        Weekday::Sat => 0,
    };
    date + Duration::days(days_until_saturday)
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
