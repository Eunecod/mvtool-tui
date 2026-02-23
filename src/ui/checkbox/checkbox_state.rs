// src/ui/checkbox/checkbox_state.rs

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{ Color, Style, Modifier};

#[derive(Debug, Default, Clone, Copy)]
pub struct CheckboxStateData
{
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
        let symbol: &str = if self.data.is_selected { "[x] " } else { "[ ] " };
        let mut style: Style = Style::default();
        
        if self.data.is_highlighted
        {
            style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
        }
        else if self.data.is_focused
        {
            style = style.fg(Color::White);
        }
        else
        {
            style = style.fg(Color::DarkGray);
        }
    
        buf.set_string(area.x, area.y, format!("{}{}", symbol, name), style);
    }
}