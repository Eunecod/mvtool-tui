// ui/checkbox/layout.rs

use crate::ui::checkbox::Checkbox;

pub trait LayoutCheckboxGroup<'a>
{
    fn new() -> Self;
    fn add_checkbox(&mut self, checkbox: Checkbox<'a>);
    fn aligment(&mut self, value: ratatui::layout::Alignment);
}

pub struct LayoutCheckboxGroupData<'a>
{
    pub checkboxes: Vec<Checkbox<'a>>,
    pub aligment: ratatui::layout::Alignment,
}
