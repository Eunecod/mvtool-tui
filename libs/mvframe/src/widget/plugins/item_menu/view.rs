// libs/mvframe/src/widget/plugins/item_menu/view.rs

use crate::widget::Widget;
use crate::widget::plugins::ItemMenuPlugins;

use imgui::Ui;

use mvcore::io::Project;

impl<'a> Widget<Vec<Project>> for ItemMenuPlugins<'a> {
    fn draw(&mut self, ui: &Ui, projects: &mut Vec<Project>) {
        mvplugin::api::bridge::imgui::CURRENT_UI.with(|cell| cell.set(Some(ui as *const Ui)));

        if let Err(error) = self
            .plugin_manager
            .emit("menu", &projects, Some(&self.data.title))
        {
            ui.text_wrapped(error);
        }

        mvplugin::api::bridge::imgui::CURRENT_UI.with(|cell| cell.set(None));
    }
}
