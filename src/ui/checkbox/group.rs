// ui/checkbox/group.rs

use ratatui::{buffer::{Buffer}, layout::Rect, text::{Line, Span, Text}, widgets::{Paragraph, Widget}};

use crate::ui::checkbox::Checkbox;
use crate::ui::checkbox::layout::LayoutCheckboxGroup;
use crate::ui::checkbox::layout::LayoutCheckboxGroupData;

pub struct VeticalCheckboxGroup<'a>(LayoutCheckboxGroupData<'a>);
impl<'a> LayoutCheckboxGroup<'a> for VeticalCheckboxGroup<'a>
{
    fn new() -> Self
    {
        return Self(LayoutCheckboxGroupData { checkboxes: Vec::new(), aligment: ratatui::layout::Alignment::Left });
    }

    fn add_checkbox(&mut self, checkbox: Checkbox<'a>)
    {
        self.0.checkboxes.push(checkbox);
    }

    fn aligment(&mut self, value: ratatui::layout::Alignment)
    {
        self.0.aligment = value;
    }
}

impl<'a> Widget for VeticalCheckboxGroup<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let mut lines: Vec<Line<'_>> = Vec::new();
        for mut checkbox in self.0.checkboxes.into_iter()
        {
            lines.push(Line::from(checkbox.get_span().clone()));
        }
        
        let paragraph: Paragraph<'_> = Paragraph::new(Text::from(lines)).alignment(self.0.aligment);
        paragraph.render(area, buf);
    }
}

pub struct HorizontalCheckboxGroup<'a>(LayoutCheckboxGroupData<'a>);
impl<'a> LayoutCheckboxGroup<'a> for HorizontalCheckboxGroup<'a>
{
    fn new() -> Self
    {
        return Self(LayoutCheckboxGroupData { checkboxes: Vec::new(), aligment: ratatui::layout::Alignment::Left });
    }

    fn add_checkbox(&mut self, checkbox: Checkbox<'a>)
    {
        self.0.checkboxes.push(checkbox);
    }

    fn aligment(&mut self, value: ratatui::layout::Alignment)
    {
        self.0.aligment = value;
    }
}

impl<'a> Widget for HorizontalCheckboxGroup<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let mut lines: Vec<Span<'_>> = Vec::new();
        for mut checkbox in self.0.checkboxes.into_iter()
        {
            lines.push(checkbox.get_span().clone());
        }
        
        let paragraph: Paragraph<'_> = Paragraph::new(Text::from(Line::from(lines))).alignment(self.0.aligment);
        paragraph.render(area, buf);
    }
}