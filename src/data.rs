use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDate,
    pub completions: Vec<NaiveDate>,
}

impl Habit {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            created_at: chrono::Local::now().date_naive(),
            completions: Vec::new(),
        }
    }

    /// Calculate current streak - consecutive days ending today or yesterday
    pub fn current_streak(&self) -> u32 {
        let today = chrono::Local::now().date_naive();
        let mut completions: Vec<NaiveDate> = self.completions.clone();
        completions.sort();

        if completions.is_empty() {
            return 0;
        }

        // Start from today or yesterday if today isn't completed
        let start_date = if completions.contains(&today) {
            today
        } else {
            let yesterday = today - chrono::Duration::days(1);
            if completions.contains(&yesterday) {
                yesterday
            } else {
                return 0;
            }
        };

        let mut streak = 0;
        let mut current_date = start_date;

        while completions.contains(&current_date) {
            streak += 1;
            current_date -= chrono::Duration::days(1);
        }

        streak
    }

    /// Calculate longest streak ever achieved
    pub fn longest_streak(&self) -> u32 {
        let mut completions: Vec<NaiveDate> = self.completions.clone();
        completions.sort();

        if completions.is_empty() {
            return 0;
        }

        let mut longest = 1;
        let mut current = 1;

        for i in 1..completions.len() {
            let diff = completions[i].signed_duration_since(completions[i - 1]).num_days();
            if diff == 1 {
                current += 1;
                longest = longest.max(current);
            } else if diff > 1 {
                current = 1;
            }
            // diff == 0 means duplicate date, ignore
        }

        longest
    }

    /// Toggle today's completion status
    pub fn toggle_today(&mut self) {
        let today = chrono::Local::now().date_naive();
        if let Some(pos) = self.completions.iter().position(|&d| d == today) {
            self.completions.remove(pos);
        } else {
            self.completions.push(today);
        }
    }

    /// Calculate completion percentage over last 3 months
    pub fn completion_percentage(&self) -> u32 {
        let today = chrono::Local::now().date_naive();
        let three_months_ago = today - chrono::Duration::days(90);

        // Count days from the later of: 3 months ago or habit creation date
        let start_date = if self.created_at > three_months_ago {
            self.created_at
        } else {
            three_months_ago
        };

        let total_days = (today - start_date).num_days() + 1;
        if total_days <= 0 {
            return 0;
        }

        let completions_in_range = self
            .completions
            .iter()
            .filter(|&date| *date >= start_date && *date <= today)
            .count() as i64;

        ((completions_in_range * 100) / total_days) as u32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub habits: Vec<Habit>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            habits: Vec::new(),
        }
    }
}
