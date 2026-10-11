// libs/mvplugin/src/core/settings.rs

use crate::core::plugin::{Plugin, PluginState};
use std::collections::HashMap;

use mvcore::io::setting::Setting;

pub struct PluginSettings<'a>(pub &'a mut HashMap<String, Plugin>);

impl<'a> Setting for PluginSettings<'a> {
    fn read(&mut self, key: &str, value: &str) {
        if let Some(plugin) = self.0.get_mut(key) {
            let mut parts = value.splitn(2, ',');

            if let Some(state_part) = parts.next() {
                match state_part.trim() {
                    "Disabled" => plugin.state = PluginState::Disabled,
                    _ => {}
                }
            }

            if let Some(shortcut_part) = parts.next() {
                let trimmed = shortcut_part.trim();
                plugin.shortcut = trimmed.parse::<u32>().ok();
            }
        }
    }

    fn write(&self, buffer: &mut String) {
        for (name, plugin) in self.0.iter() {
            let state_str = match &plugin.state {
                PluginState::Active(_) => "Active",
                PluginState::Disabled => "Disabled",
            };

            let shortcut_str = plugin
                .shortcut
                .map(|sc| sc.to_string())
                .unwrap_or_else(|| "None".to_string());

            buffer.push_str(&format!("{}={},{}\n", name, state_str, shortcut_str));
        }
    }
}
