// libs/mvcore/src/adapter/clipboard.rs

pub struct AdapterClipboard;

impl AdapterClipboard {
    pub fn get(&mut self) -> Option<String> {
        arboard::Clipboard::new()
            .ok()
            .and_then(|mut cb| cb.get_text().ok())
    }

    pub fn set(&mut self, text: &str) {
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text(text.to_owned());
        }
    }
}
