// src/ui/checkbox/checkbox.rs

use crate::ui::checkbox::CheckboxState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

pub struct Checkbox<'a>
{
    pub name: String,
    pub state: CheckboxState,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Checkbox<'a>
{
    pub fn new(name: impl Into<String>) -> Self
    {
        return Self { name: name.into(), state: CheckboxState::default(), phantom: std::marker::PhantomData };
    }

    pub fn set_state(&mut self, state: CheckboxState)
    {
        self.state = state;
    }
}

impl<'a> Widget for Checkbox<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        self.state.render(area, buf, &self.name);
    }
}