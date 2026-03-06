// src/ui/messagelog/messagelog.rs

use ratatui::{ style::{ Color, Style }, widgets::Paragraph };

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MessageType
{
    #[default]
    Warning,
    Success,
    Error,
    Info,
}

pub struct MessageLog
{
    message: String,
    message_type: MessageType,
}

impl MessageLog
{   
    pub fn new() -> Self
    {
        return Self { message: String::new(), message_type: MessageType::Info };
    }

    pub fn add_message(&mut self, message: String, message_type: MessageType)
    {
        self.message_type = message_type;
        self.message = message;
    }

    pub fn get_message(&self) -> Paragraph
    {
        let (style, prefix) = match self.message_type
        {
            MessageType::Warning =>
            {
                (Style::default().fg(Color::Yellow).bold(), "[warning]:".to_string())
            }
            MessageType::Success =>
            {
                (Style::default().fg(Color::Green).bold(), "[success]:".to_string())
            }
            MessageType::Error =>
            {
                (Style::default().fg(Color::Red).bold(), "[error]:".to_string())
            }
            MessageType::Info =>
            {
                (Style::default().fg(Color::LightBlue).bold(), "[info]:".to_string())
            }
        };
        
        return Paragraph::new(format!("{} {}", prefix, self.message)).style(style);
    }
}