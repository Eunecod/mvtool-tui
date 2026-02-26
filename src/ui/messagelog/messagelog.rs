// src/ui/messagelog/messagelog.rs

use ratatui::{ style::{ Color, Style }, widgets::Paragraph };
use std::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MessageType
{
    #[default]
    Warning,
    Success,
    Error,
    Info,
}

pub struct LogEvent
{
    pub message: String,
    pub message_type: MessageType,
}

#[derive(Debug)]
pub struct MessageLog
{
    message: String,
    message_type: MessageType,
    sender: mpsc::Sender<LogEvent>,
    receiver: mpsc::Receiver<LogEvent>
}

impl MessageLog
{   
    pub fn new() -> Self
    {
        let (sender, receiver) = mpsc::channel();
        return Self { message: String::new(), message_type: MessageType::Info, sender, receiver };
    }

    pub fn update(&mut self)
    {
        while let Ok(event) = self.receiver.try_recv()
        {
            self.add_message(event.message, event.message_type);
        }
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

    pub fn get_sender(&self) -> mpsc::Sender<LogEvent>
    {
        self.sender.clone()
    }

}