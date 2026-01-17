use crate::data::AppData;

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
}

impl App {
    pub fn new(data: AppData) -> Self {
        Self {
            data,
            should_quit: false,
            selected_index: 0,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn select_next(&mut self) {
        if self.data.habits.is_empty() {
            return;
        }
        if self.selected_index < self.data.habits.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
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
