// libs/mvplugin/src/api/imgui.rs

use crate::loader::RegisterContext;

use imgui::Ui;

use std::cell::Cell;

thread_local! {
    pub static CURRENT_UI: Cell<Option<*const Ui>> = const { Cell::new(None) };
}

pub unsafe fn with_ui<F, R>(f: F) -> mlua::Result<R>
where
    F: FnOnce(&Ui) -> mlua::Result<R>,
{
    CURRENT_UI.with(|cell| {
        if let Some(ptr) = cell.get() {
            unsafe { f(&*ptr) }
        } else {
            Err(mlua::Error::runtime(
                "Контекст imgui недоступен вне кадра отрисовки",
            ))
        }
    })
}

pub fn imgui(ctx: &mut RegisterContext) -> Result<(), String> {
    let menu_item = ctx
        .lua
        .create_function(|_, item_name: String| unsafe {
            with_ui(|ui| {
                let clicked = ui.menu_item(item_name);

                Ok(clicked)
            })
        })
        .map_err(|error| format!("Ошибка загрузки [api: menu_item]: {}", error))?;

    let text = ctx
        .lua
        .create_function(|_, text: String| unsafe {
            with_ui(|ui| {
                ui.text_wrapped(text);
                Ok(())
            })
        })
        .map_err(|error| format!("Ошибка загрузки [api: imgui_text]: {}", error))?;

    let button = ctx
        .lua
        .create_function(|_, text: String| unsafe {
            with_ui(|ui| {
                let clicked = ui.button_with_size(text, [-1.0, 0.0]);
                Ok(clicked)
            })
        })
        .map_err(|error| format!("Ошибка загрузки [api: imgui_button]: {}", error))?;

    ctx.api
        .set("menu_item", menu_item)
        .map_err(|error| error.to_string())?;

    ctx.api
        .set("text", text)
        .map_err(|error| error.to_string())?;

    ctx.api
        .set("button", button)
        .map_err(|error| error.to_string())?;

    Ok(())
}
