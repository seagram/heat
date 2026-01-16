use crate::data::AppData;

pub struct App {
    pub data: AppData,
    pub should_quit: bool,
}

impl App {
    pub fn new(data: AppData) -> Self {
        Self {
            data,
            should_quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
