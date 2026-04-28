// source/widgets/mod.rs

use imgui::Ui;

pub trait Widget<T> {
    fn draw(&mut self, ui: &Ui, data: &mut T);
}

pub mod helper;

pub mod console;
pub mod projects;
pub mod configures;
pub mod components;
pub mod scripts;
pub mod about;
pub mod messagebox;