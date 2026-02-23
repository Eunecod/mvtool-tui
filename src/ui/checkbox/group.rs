// src/ui/checkbox/group.rs

use ratatui::buffer::Buffer;
use ratatui::layout::{ Rect, Alignment };
use ratatui::widgets::Widget;
use crate::ui::checkbox::Checkbox;
use crate::ui::checkbox::layout::{ LayoutCheckboxGroup, LayoutCheckboxGroupData };

pub struct HorizontalCheckboxGroup<'a>(LayoutCheckboxGroupData<'a>);
pub struct VerticalCheckboxGroup<'a>(LayoutCheckboxGroupData<'a>);

impl<'a> LayoutCheckboxGroup<'a> for HorizontalCheckboxGroup<'a>
{
    fn new() -> Self
    { 
        return Self(LayoutCheckboxGroupData::default());
    }

    fn add_checkbox(&mut self, checkbox: Checkbox<'a>)
    {
        self.0.checkboxes.push(checkbox);
    }

    fn alignment(&mut self, value: Alignment)
    {
        self.0.alignment = value;
    }
}

impl<'a> Widget for HorizontalCheckboxGroup<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let mut x: u16 = area.x;
        for checkbox in self.0.checkboxes
        {
            let width = checkbox.name.len() as u16 + 4; 
            if x + width > area.right()
            {
                break;
            }

            Widget::render(checkbox, Rect::new(x, area.y, width, 1), buf);
            x += width + 2;
        }
    }
}

impl<'a> LayoutCheckboxGroup<'a> for VerticalCheckboxGroup<'a>
{
    fn new() -> Self
    {
        return Self(LayoutCheckboxGroupData::default());
    }

    fn add_checkbox(&mut self, checkbox: Checkbox<'a>)
    {
        self.0.checkboxes.push(checkbox);
    }

    fn alignment(&mut self, value: Alignment)
    {
        self.0.alignment = value;
    }
}

impl<'a> Widget for VerticalCheckboxGroup<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        for (i, checkbox) in self.0.checkboxes.into_iter().enumerate()
        {
            let y = area.y + i as u16;
            if y >= area.bottom()
            {
                break;
            }
            
            Widget::render(checkbox, Rect::new(area.x, y, area.width, 1), buf);
        }
    }
}