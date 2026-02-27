// src/ui/checkbox/group.rs

use ratatui::buffer::Buffer;
use ratatui::layout::{ Rect, Alignment };
use ratatui::widgets::{StatefulWidget, Widget, ScrollbarOrientation, Scrollbar, ScrollbarState};
use crate::ui::checkbox::Checkbox;
use crate::ui::checkbox::layout::{ LayoutCheckboxGroup, LayoutCheckboxGroupData };

#[derive(Debug, Default)]
pub struct CheckboxGroupState
{
    pub cursor: usize,
    pub scroll_offset: usize,
}

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

impl<'a> StatefulWidget for HorizontalCheckboxGroup<'a>
{
    type State = CheckboxGroupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    {
        let total_items: usize = self.0.checkboxes.len();
        if total_items == 0
        {
            return;
        }

        let visible_count: usize = area.width as usize / total_items;

        if state.cursor >= state.scroll_offset + visible_count
        {
            state.scroll_offset = (state.cursor + 1).saturating_sub(visible_count);
        }
        else if state.cursor < state.scroll_offset
        {
            state.scroll_offset = state.cursor;
        }

        let mut x: u16 = area.x;
        for checkbox in self.0.checkboxes.into_iter().skip(state.scroll_offset)
        {
            let width: u16 = checkbox.name.len() as u16 + 4;
            if x + width > area.right()
            {
                break;
            }

            Widget::render(checkbox, Rect::new(x, area.y, width, 1), buf);
            x += width.saturating_add(1) ;
        }

        let scrollbar: Scrollbar<'_> = Scrollbar::new(ScrollbarOrientation::HorizontalBottom).begin_symbol(Some("┠")).end_symbol(Some("┨")).track_symbol(Some("─")).thumb_symbol("═");
        StatefulWidget::render(scrollbar, area, buf, &mut ScrollbarState::new(total_items).position(state.cursor).viewport_content_length(visible_count));
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

impl<'a> StatefulWidget for VerticalCheckboxGroup<'a>
{
    type State = CheckboxGroupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    {
        let total_items: usize = self.0.checkboxes.len();
        let height: usize = area.height as usize;

        if total_items == 0
        {
            return;
        }

        if state.cursor >= state.scroll_offset + height
        {
            state.scroll_offset = state.cursor - height + 1;
        }
        else if state.cursor < state.scroll_offset
        {
            state.scroll_offset = state.cursor;
        }

        for (i, checkbox) in self.0.checkboxes.into_iter().skip(state.scroll_offset).enumerate()
        {
            let y: u16 = area.y + i as u16;
            if y >= area.bottom()
            {
                break;
            }
            
            Widget::render(checkbox, Rect::new(area.x, y, area.width.saturating_sub(1), 1), buf);
        }

        let scrollbar: Scrollbar<'_> = Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("┯")).end_symbol(Some("┷")).track_symbol(Some("│")).thumb_symbol("║");
        StatefulWidget::render(scrollbar, area, buf, &mut ScrollbarState::new(total_items).position(state.cursor).viewport_content_length(height));
    }
}