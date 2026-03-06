// src/ui/checkbox/layout.rs

use crate::ui::checkbox::Checkbox;

pub trait LayoutCheckboxGroup<'a>
{
    fn new() -> Self;
    fn add_checkbox(&mut self, checkbox: Checkbox<'a>);
}

pub struct LayoutCheckboxGroupData<'a>
{
    pub checkboxes: Vec<Checkbox<'a>>,
}

impl<'a> Default for LayoutCheckboxGroupData<'a>
{
    fn default() -> Self
    {
        return Self { checkboxes: Vec::new() };
    }
}