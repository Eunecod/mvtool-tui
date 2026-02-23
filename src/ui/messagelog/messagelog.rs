// src/ui/messagelog/messagelog.rs

use ratatui::{ style::{ Color, Style }, text::Text, widgets::Paragraph };

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MessageType
{
    #[default]
    Warning,
    Success,
    Error,
    Info,
}

#[derive(Debug, Default)]
pub struct MessageLog
{
    message: String,
    style: Style,
}

impl MessageLog
{

    pub fn add_message(&mut self, message: String, message_type: MessageType)
    {
        let prefix: String;
        match message_type
        {
            MessageType::Warning =>
            {
                self.style = Style::default().fg(Color::Yellow).bold();
                prefix = "[Waring]:".to_string();
            }
            MessageType::Success =>
            {
                self.style = Style::default().fg(Color::Green).bold();
                prefix = "[Success]:".to_string();
            }
            MessageType::Error =>
            {
                self.style = Style::default().fg(Color::Red).bold();
                prefix = "[Error]:".to_string();
            }
            MessageType::Info =>
            {
                self.style = Style::default().fg(Color::LightCyan).bold();
                prefix = "[Info]:".to_string();
            }
        };

        self.message = format!("{} {}", prefix, message);
    }

    pub fn get_message(&self) -> Paragraph
    {
        return Paragraph::new(Text::from(self.message.clone())).style(self.style);
    }

}