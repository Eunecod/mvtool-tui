// libs/mvframe/src/widget/messagebox/data.rs

pub struct MessageBoxData {
    pub title: String,
    pub message: String,
    pub action: Box<dyn FnOnce()>,
}

impl MessageBoxData {
    pub fn new<F>(title: &str, message: &str, action: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        Self {
            title: title.into(),
            message: message.into(),
            action: Box::new(action),
        }
    }
}

impl Default for MessageBoxData {
    fn default() -> Self {
        Self {
            title: String::new(),
            message: String::new(),
            action: Box::new(|| {}),
        }
    }
}
