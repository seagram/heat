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
}
