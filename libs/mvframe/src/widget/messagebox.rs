// libs/mvframe/src/widgets/messagebox.rs

pub mod data;
use data::MessageBoxData;

mod view;

pub struct MessageBox {
    pub data: Option<MessageBoxData>,
    is_open: bool,
}

impl MessageBox {
    pub fn new() -> Self {
        Self {
            data: Some(MessageBoxData::default()),
            is_open: false,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        if let Some(ref mut data) = self.data {
            data.title = title;
        }
        self
    }

    pub fn message(mut self, message: String) -> Self {
        if let Some(ref mut data) = self.data {
            data.message = message;
        }
        self
    }

    pub fn spawn<F>(mut self, action: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        if let Some(ref mut data) = self.data {
            data.action = Box::new(action);
        }
        self.is_open = true;
        self
    }

    pub fn click_ok(&mut self) {
        self.is_open = false;
        if let Some(data) = self.data.take() {
            (data.action)();
        }
    }

    pub fn click_cancel(&mut self) {
        self.is_open = false;
        self.data = None;
    }
}
