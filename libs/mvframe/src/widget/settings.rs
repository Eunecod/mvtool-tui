// libs/mvframe/src/widget/settings.rs

pub mod data;
pub use data::SettingsData;

mod view;

pub struct SettingsWidget<'a> {
    pub data: SettingsData<'a>,
    pub is_open: bool,
}
