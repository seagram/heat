use chrono::{Datelike, Duration, NaiveDate, Weekday};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, InputMode, GRID_COLUMNS};
use crate::data::Habit;

const CARD_HEIGHT_WITH_STATS: u16 = 10;
const CARD_HEIGHT_NO_STATS: u16 = 9;

pub fn card_height(show_stats: bool) -> u16 {
    if show_stats {
        CARD_HEIGHT_WITH_STATS
    } else {
        CARD_HEIGHT_NO_STATS
    }
}

/// Truncate a string to fit within max_width, adding "..." if truncated
fn truncate_name(name: &str, max_width: usize) -> String {
    if name.chars().count() <= max_width {
        name.to_string()
    } else if max_width <= 3 {
        name.chars().take(max_width).collect()
    } else {
        let truncated: String = name.chars().take(max_width - 3).collect();
        format!("{}...", truncated)
    }
}

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

    // Render popup if in adding, renaming, or deleting mode
    if app.input_mode == InputMode::Adding {
        render_add_popup(frame, app, area);
    } else if app.input_mode == InputMode::Renaming {
        render_rename_popup(frame, app, area);
    } else if app.input_mode == InputMode::Deleting {
        render_delete_popup(frame, app, area);
    }
}

fn render_habit_list(frame: &mut Frame, app: &App, area: Rect) {
    let total_rows = app.total_rows();
    if total_rows == 0 {
        return;
    }

    let current_card_height = card_height(app.show_stats);

    // Calculate how many rows can fit in the visible area
    let visible_rows = (area.height / current_card_height).max(1) as usize;

    // Get the range of rows to render based on scroll offset
    let start_row = app.scroll_offset;
    let end_row = (start_row + visible_rows + 1).min(total_rows);

    // Create row constraints
    let row_constraints: Vec<Constraint> = (start_row..end_row)
        .map(|_| Constraint::Length(current_card_height))
        .collect();

    let row_areas = Layout::vertical(row_constraints).split(area);

    // Create column constraints (equal width for each column)
    let col_constraints: Vec<Constraint> = (0..GRID_COLUMNS)
        .map(|_| Constraint::Ratio(1, GRID_COLUMNS as u32))
        .collect();

    for (row_offset, row_area) in row_areas.iter().enumerate() {
        let row = start_row + row_offset;
        let col_areas = Layout::horizontal(col_constraints.clone()).split(*row_area);

        for col in 0..GRID_COLUMNS {
            let habit_index = row * GRID_COLUMNS + col;
            if habit_index < app.data.habits.len() {
                let habit = &app.data.habits[habit_index];
                let is_selected = habit_index == app.selected_index;
                render_habit_card(frame, habit, col_areas[col], is_selected, app.show_stats);
            }
        }
    }
}

fn render_habit_card(frame: &mut Frame, habit: &Habit, area: Rect, is_selected: bool, show_stats: bool) {
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    // Truncate name to fit within card width (minus borders and padding)
    let max_name_width = area.width.saturating_sub(6) as usize;
    let display_name = truncate_name(&habit.name, max_name_width);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!(" {} ", display_name));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if show_stats {
        let content_layout = Layout::vertical([
            Constraint::Length(1), // Stats row
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
            Span::raw(streak_text),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::raw(format!("Best: {}", longest_streak)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::raw(format!("{}%", completion_pct)),
        ]))
        .centered();
        frame.render_widget(stats, content_layout[0]);

        // Heatmap grid (with day labels when stats are shown)
        let heatmap_lines = build_heatmap(habit, content_layout[1].width, true);
        let heatmap = Paragraph::new(heatmap_lines);
        frame.render_widget(heatmap, content_layout[1]);
    } else {
        // Just render the heatmap (no day labels)
        let heatmap_lines = build_heatmap(habit, inner_area.width, false);
        let heatmap = Paragraph::new(heatmap_lines);
        frame.render_widget(heatmap, inner_area);
    }
}

fn build_heatmap(habit: &Habit, width: u16, show_day_labels: bool) -> Vec<Line<'static>> {
    let today = chrono::Local::now().date_naive();

    // Calculate how many week columns can fit in the available width
    // With day labels: "S " (2 chars) + n cells (1 char each) + (n-1) spaces = 2 + 2n - 1 = 2n + 1
    // Without day labels: n cells (1 char each) + (n-1) spaces = 2n - 1, minus 1 for padding
    let num_weeks = if show_day_labels {
        ((width.saturating_sub(3)) / 2).max(1) as usize
    } else {
        // Subtract 1 from max to leave comfortable margins
        ((width.saturating_add(1)) / 2).saturating_sub(1).max(1) as usize
    };

    // Calculate left padding to center the grid
    let left_padding = if show_day_labels {
        // With day labels: "S " (2 chars) + grid (2 * num_weeks - 1)
        let content_width = (2 + num_weeks * 2).saturating_sub(1) as u16;
        ((width.saturating_sub(content_width)) / 2) as usize
    } else {
        // Grid width = num_weeks cells + (num_weeks - 1) spaces = 2 * num_weeks - 1
        let grid_width = (num_weeks * 2).saturating_sub(1) as u16;
        ((width.saturating_sub(grid_width)) / 2) as usize
    };

    // Find the Saturday at or after today to end the grid
    let end_date = find_next_saturday(today);

    // Calculate start date: Sunday of the first week
    // end_date is Saturday, so Sunday of that week is end_date - 6
    // Then go back (num_weeks - 1) full weeks
    let start_date = end_date - Duration::days(6 + (num_weeks as i64 - 1) * 7);

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
        let mut spans: Vec<Span<'static>> = Vec::new();

        // Add left padding to center the grid when not showing day labels
        if left_padding > 0 {
            spans.push(Span::raw(" ".repeat(left_padding)));
        }

        // Only add day label if show_day_labels is true
        if show_day_labels {
            spans.push(Span::raw(format!("{} ", label)));
        }

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
        Span::styled("h/j/k/l", Style::default().fg(Color::Yellow)),
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
        separator.clone(),
        Span::styled("s", Style::default().fg(Color::Yellow)),
        Span::raw(": stats"),
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

fn render_add_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup_width = 32;
    let popup_height = 6;

    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Add New Habit ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let layout = Layout::vertical([
        Constraint::Length(1), // empty line
        Constraint::Length(1), // input line
        Constraint::Length(1), // empty line
        Constraint::Length(1), // help line
    ])
    .split(inner);

    // Input line
    let input_line = Line::from(vec![
        Span::raw("  Name: "),
        Span::styled(
            format!("{}_", app.input_buffer),
            Style::default().fg(Color::White),
        ),
    ]);
    frame.render_widget(Paragraph::new(input_line), layout[1]);

    // Help line
    let help = Line::from(vec![
        Span::styled("  Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": confirm  "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": cancel"),
    ]);
    frame.render_widget(Paragraph::new(help), layout[3]);
}

fn render_rename_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup_width = 32;
    let popup_height = 6;

    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Rename Habit ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let layout = Layout::vertical([
        Constraint::Length(1), // empty line
        Constraint::Length(1), // input line
        Constraint::Length(1), // empty line
        Constraint::Length(1), // help line
    ])
    .split(inner);

    // Input line
    let input_line = Line::from(vec![
        Span::raw("  Name: "),
        Span::styled(
            format!("{}_", app.input_buffer),
            Style::default().fg(Color::White),
        ),
    ]);
    frame.render_widget(Paragraph::new(input_line), layout[1]);

    // Help line
    let help = Line::from(vec![
        Span::styled("  Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": confirm  "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": cancel"),
    ]);
    frame.render_widget(Paragraph::new(help), layout[3]);
}

fn render_delete_popup(frame: &mut Frame, app: &App, area: Rect) {
    let habit_name = app
        .data
        .habits
        .get(app.selected_index)
        .map(|h| truncate_name(&h.name, 17))
        .unwrap_or_default();

    let popup_width = 32;
    let popup_height = 6;

    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Delete Habit ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let layout = Layout::vertical([
        Constraint::Length(1), // empty line
        Constraint::Length(1), // prompt line
        Constraint::Length(1), // empty line
        Constraint::Length(1), // help line
    ])
    .split(inner);

    // Prompt line
    let prompt = Line::from(format!("  Delete \"{}\"?", habit_name));
    frame.render_widget(Paragraph::new(prompt), layout[1]);

    // Help line
    let help = Line::from(vec![
        Span::styled("  y", Style::default().fg(Color::Yellow)),
        Span::raw(": yes  "),
        Span::styled("n", Style::default().fg(Color::Yellow)),
        Span::raw(": no"),
    ]);
    frame.render_widget(Paragraph::new(help), layout[3]);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
