// libs/mvplugin/src/api/bridge/imgui.rs

use mlua::Result;

use imgui::Ui;

use std::cell::Cell;

thread_local! {
    pub static CURRENT_UI: Cell<Option<*const Ui>> = const { Cell::new(None) };
}

pub unsafe fn with_ui<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&Ui) -> Result<R>,
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
