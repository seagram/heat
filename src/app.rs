use crate::data::AppData;

pub const GRID_COLUMNS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Adding,
    Renaming,
    Deleting,
}

pub struct App {
    pub data: AppData,
    pub should_quit: bool,
    pub selected_index: usize,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub scroll_offset: usize, // Now represents row offset, not card offset
}

impl App {
    pub fn new(data: AppData) -> Self {
        Self {
            data,
            should_quit: false,
            selected_index: 0,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            scroll_offset: 0,
        }
    }

    /// Get the (row, col) position for a given linear index
    pub fn grid_position(&self, index: usize) -> (usize, usize) {
        (index / GRID_COLUMNS, index % GRID_COLUMNS)
    }

    /// Get the current selection's row
    pub fn selected_row(&self) -> usize {
        self.selected_index / GRID_COLUMNS
    }

    /// Get the total number of rows in the grid
    pub fn total_rows(&self) -> usize {
        if self.data.habits.is_empty() {
            0
        } else {
            (self.data.habits.len() + GRID_COLUMNS - 1) / GRID_COLUMNS
        }
    }

    pub fn adjust_scroll(&mut self, visible_height: u16, card_height: u16) {
        if card_height == 0 {
            return;
        }
        let visible_rows = (visible_height / card_height).max(1) as usize;
        let selected_row = self.selected_row();

        // If selection is above visible area, scroll up
        if selected_row < self.scroll_offset {
            self.scroll_offset = selected_row;
        }

        // If selection is below visible area, scroll down
        if selected_row >= self.scroll_offset + visible_rows {
            self.scroll_offset = selected_row.saturating_sub(visible_rows - 1);
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Move down one row (j key)
    pub fn select_down(&mut self) {
        if self.data.habits.is_empty() {
            return;
        }
        let new_index = self.selected_index + GRID_COLUMNS;
        if new_index < self.data.habits.len() {
            self.selected_index = new_index;
        }
    }

    /// Move up one row (k key)
    pub fn select_up(&mut self) {
        if self.selected_index >= GRID_COLUMNS {
            self.selected_index -= GRID_COLUMNS;
        }
    }

    /// Move left one column (h key)
    pub fn select_left(&mut self) {
        let (_, col) = self.grid_position(self.selected_index);
        if col > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move right one column (l key)
    pub fn select_right(&mut self) {
        if self.data.habits.is_empty() {
            return;
        }
        let (_, col) = self.grid_position(self.selected_index);
        if col < GRID_COLUMNS - 1 && self.selected_index + 1 < self.data.habits.len() {
            self.selected_index += 1;
        }
    }

    pub fn select_first(&mut self) {
        self.selected_index = 0;
    }

    pub fn select_last(&mut self) {
        if !self.data.habits.is_empty() {
            self.selected_index = self.data.habits.len() - 1;
        }
    }

    pub fn toggle_today(&mut self) {
        if let Some(habit) = self.data.habits.get_mut(self.selected_index) {
            habit.toggle_today();
        }
    }

    pub fn start_adding(&mut self) {
        self.input_mode = InputMode::Adding;
        self.input_buffer.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn confirm_add(&mut self) {
        let name = self.input_buffer.trim().to_string();
        if !name.is_empty() {
            let habit = crate::data::Habit::new(name);
            self.data.habits.push(habit);
            self.selected_index = self.data.habits.len() - 1;
        }
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn start_renaming(&mut self) {
        if let Some(habit) = self.data.habits.get(self.selected_index) {
            self.input_buffer = habit.name.clone();
            self.input_mode = InputMode::Renaming;
        }
    }

    pub fn confirm_rename(&mut self) {
        let name = self.input_buffer.trim().to_string();
        if !name.is_empty() {
            if let Some(habit) = self.data.habits.get_mut(self.selected_index) {
                habit.name = name;
            }
        }
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn start_deleting(&mut self) {
        if !self.data.habits.is_empty() {
            self.input_mode = InputMode::Deleting;
        }
    }

    pub fn confirm_delete(&mut self) {
        if self.selected_index < self.data.habits.len() {
            self.data.habits.remove(self.selected_index);
            // Adjust selected_index if we deleted the last item
            if self.selected_index >= self.data.habits.len() && !self.data.habits.is_empty() {
                self.selected_index = self.data.habits.len() - 1;
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn cancel_delete(&mut self) {
        self.input_mode = InputMode::Normal;
    }
}
