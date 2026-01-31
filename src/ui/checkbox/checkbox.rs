// ui/checkbox/checkbox.rs

use ratatui::{style::{Color, Style}, text::{Line, Span}};

#[derive(Debug, Clone, Default)]
pub struct CheckboxState
{
    checked: bool,
    focused: bool,
    highlight: bool,

    enable_signed_highlight: bool
}

impl CheckboxState
{
    pub fn new(checked: bool) -> Self
    {
        return Self { checked: checked, focused: false, highlight: false, enable_signed_highlight: true};
    }

    // fn set_checked(&mut self, checked: bool)
    // {
    //     self.checked = checked;
    // }

    pub fn set_enable_signed_highlight(&mut self, value: bool)
    {
        self.enable_signed_highlight = value;
    }

    // fn is_checked(&self) -> bool
    // {
    //     return self.checked;
    // }

    pub fn focus(&mut self)
    {
        self.focused = true;
    }

    pub fn highlight(&mut self)
    {
        self.highlight = true;
    }
}

pub struct Checkbox<'a>
{
    label: Line<'a>,
    state: CheckboxState,

    style_highlight: Style,
    style_focus:     Style,
    style_unfocus:   Style,
}

impl<'a> Checkbox<'a>
{
    pub fn new(label: &'a str) -> Self
    {
        return Self
        { 
            label: Line::from(label),
            state: CheckboxState::new(false),

            style_highlight: Style::default().fg(Color::Green).bold(),
            style_focus:     Style::default().fg(Color::Gray),
            style_unfocus:   Style::default().fg(Color::DarkGray),
        };
    }

    pub fn set_state(&mut self, state: CheckboxState)
    {
        self.state = state;
    }

    pub fn get_span(&mut self) -> Span<'a>
    {
        let mut style: Style;
        if self.state.focused
        {
            style = self.style_focus;
            if self.state.highlight
            {
                style = self.style_highlight;
            }
        }
        else
        {
            style = self.style_unfocus;   
        }

        let state_span: Span<'_>     = Span::default().content(if self.state.checked { "[x]" } else { "[ ]" }).style(style);
        let highlight_span: Span<'_> = Span::default().content(if self.state.enable_signed_highlight && self.state.highlight { ">" } else { " " }).style(style);

        let checkbox_span: Span<'_> = Span::styled(format!("{} {} {}", highlight_span, state_span, self.label), style);

        return checkbox_span;
    }
}