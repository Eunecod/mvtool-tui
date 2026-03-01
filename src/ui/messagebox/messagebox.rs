// src/ui/messagebox/messagebox.rs

use ratatui::{ buffer::Buffer, layout::{ Rect, Layout, Alignment, Constraint }, widgets::{ Block, Borders, Widget, Padding, Paragraph }, text::{ Line, Text }, style::Stylize };

pub struct MessageBox<'a>
{
    title: String,
    message_body: Text<'a>,
    on_accept: Option<Box<dyn Fn() + 'a>>,
    on_reject: Option<Box<dyn Fn() + 'a>>,
}

impl<'a> MessageBox<'a>
{
    pub fn new<Faccept, Freject>(title: &str, message_body: Text<'a>, on_accept: Faccept, on_reject: Freject) -> Self
    where 
        Faccept: Fn() + 'a,
        Freject: Fn() + 'a,
    {
        return Self { title: title.into(), message_body: message_body, on_accept: Some(Box::new(on_accept)), on_reject: Some(Box::new(on_reject)) };
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
        ratatui::widgets::Clear.render(area, buf);

        let block: Block<'_> = Block::default().title(self.title.clone()).borders(Borders::ALL).padding(Padding::horizontal(1));
        let [body_area, footer_area] = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(block.inner(area));
        
        block.render(area, buf);
        self.message_body.clone().render(body_area, buf);
        Paragraph::new(Line::from(vec![" Enter ".black().on_gray(), " ".into(), " Esc ".black().on_gray()])).alignment(Alignment::Right).render(footer_area, buf);
    }
}