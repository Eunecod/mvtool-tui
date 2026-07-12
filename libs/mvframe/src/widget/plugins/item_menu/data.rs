// libs/mvframe/src/widget/plugins/item_menu/data.rs

pub struct ItemMenuData {
    pub title: String,
}

impl ItemMenuData {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Default for ItemMenuData {
    fn default() -> Self {
        Self {
            title: String::new(),
        }
    }
}
