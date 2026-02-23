// src/ui/checkbox/layout.rs

use crate::ui::checkbox::Checkbox;
use ratatui::layout::Alignment;

#[allow(dead_code)]
pub trait LayoutCheckboxGroup<'a>
{
    fn new() -> Self;
    fn add_checkbox(&mut self, checkbox: Checkbox<'a>);
    fn alignment(&mut self, value: Alignment);
}

pub struct LayoutCheckboxGroupData<'a>
{
    pub checkboxes: Vec<Checkbox<'a>>,
    pub alignment: Alignment,
}

impl<'a> Default for LayoutCheckboxGroupData<'a>
{
    fn default() -> Self
    {
        return Self { checkboxes: Vec::new(), alignment: Alignment::Left };
    }
}