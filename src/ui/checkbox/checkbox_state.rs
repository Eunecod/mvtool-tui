// src/ui/checkbox/checkbox_state.rs

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{ Color, Style, Modifier };

#[derive(Debug, Default, Clone, Copy)]
pub struct CheckboxStateData
{
    pub style_highlighted: Option<Style>,
    pub symbols: Option<(&'static str, &'static str)>,
    pub is_selected: bool,  
    pub is_focused: bool,
    pub is_highlighted: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CheckboxState
{
    pub data: CheckboxStateData,
}

impl CheckboxState
{
    pub fn new(selected: bool) -> Self
    {
        return Self { data: CheckboxStateData { is_selected: selected, ..Default::default() } };
    }

    pub fn focus(&mut self)
    {
        self.data.is_focused = true;
    }

    pub fn highlight(&mut self)
    {
        self.data.is_highlighted = true;
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, name: &str)
    {
        let mut style: Style = Style::default();
        if self.data.is_highlighted
        {
            style = self.data.style_highlighted.unwrap_or_else(|| Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
        }
        else if self.data.is_focused
        {
            style = style.fg(Color::White);
        }
        else
        {
            style = style.fg(Color::DarkGray);
        }
    
        let (unchecked, checked) = self.data.symbols.unwrap_or(("[ ]", "[■]"));
        buf.set_string(area.x, area.y, format!("{} {}", if self.data.is_selected { checked } else { unchecked }, name), style);
    }
}