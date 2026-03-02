// src/ui/messagebox/messagebox.rs

use ratatui::{ buffer::Buffer, layout::{ Rect, Layout, Alignment, Constraint, Direction }, widgets::{ Block, Borders, Widget, Padding, Paragraph }, text::{ Line, Text }, style::Stylize };

pub struct MessageBox<'a>
{
    title: String,
    message_body: Text<'a>,
    percent_y: u16,
    percent_x: u16,
    on_accept: Option<Box<dyn Fn() + 'a>>,
    on_reject: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MessageBox<'a>
{
    pub fn new(title: &str, message_body: Text<'a>) -> Self
    {
        return Self { title: title.into(), message_body: message_body, on_accept: None, on_reject: None, percent_x: 100, percent_y: 100 };
    }

    pub fn set_accept<F>(mut self, on_accept: F) -> Self
    where 
        F: Fn() + 'a,
    {
        self.on_accept = Some(Box::new(on_accept));

        return self;
    }

    pub fn set_reject<F>(mut self, on_reject: F) -> Self
    where 
        F: Fn() + 'a,
    {
        self.on_reject = Some(Box::new(on_reject));

        return self;
    }

    pub fn with_size(mut self, x: u16, y: u16) -> Self
    {
        self.percent_x = x;
        self.percent_y = y;
        
        return self
    }

    pub fn accept(&self)
    {
        if let Some(callback) = &self.on_accept
        {
            callback();
        }
    }

    pub fn reject(&self)
    {
        if let Some(callback) = &self.on_reject
        {
            callback();
        }
    }
}

impl<'a> Widget for &MessageBox<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let popup_layout: std::rc::Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - self.percent_y) / 2),
                Constraint::Percentage(self.percent_y),
                Constraint::Percentage((100 - self.percent_y) / 2),
            ]
        ).split(area);

        let messagebox_area: Rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - self.percent_x) / 2),
                Constraint::Percentage(self.percent_x),
                Constraint::Percentage((100 - self.percent_x) / 2),
            ]
        ).split(popup_layout[1])[1];

        ratatui::widgets::Clear.render(messagebox_area, buf);

        let block: Block<'_> = Block::default().title(self.title.clone()).borders(Borders::ALL).padding(Padding::horizontal(1));
        let [body_area, footer_area] = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(block.inner(messagebox_area));
        
        block.render(messagebox_area, buf);
        self.message_body.clone().render(body_area, buf);
        Paragraph::new(Line::from(vec![" Enter ".black().on_gray(), " ".into(), " Esc ".black().on_gray()])).alignment(Alignment::Right).render(footer_area, buf);
    }
}