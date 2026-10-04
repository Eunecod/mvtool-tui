// libs/mvframe/src/widget/plugin/view.rs

use crate::widget::Widget;
use crate::widget::plugin::PluginWidget;

use imgui::Condition;
use imgui::Ui;

use mvcore::io::schema::Root;
use mvplugin::core::models::Root as LuaRoot;

impl<'a> Widget<Root> for PluginWidget<'a> {
    fn draw(&mut self, ui: &Ui, data: &mut Root) {
        mvplugin::api::imgui::CURRENT_UI.with(|cell| cell.set(Some(ui)));

        let header = format!(
            "{}###{}_plugin_window",
            self.data.plugin.metadata.name, self.data.plugin.metadata.name
        );
        ui.window(&header)
            .size([500.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.separator();

                if let Err(error) = self.data.plugin.emit("build", LuaRoot(data.clone())) {
                    ui.text_wrapped(error);
                }
            });

        mvplugin::api::imgui::CURRENT_UI.with(|cell| cell.set(None));
    }
}
