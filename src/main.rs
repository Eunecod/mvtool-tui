use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::{Rect}, style::Stylize, symbols::border, text::{Line}, widgets::{Block, Widget}, DefaultTerminal, Frame};

fn main() -> io::Result<()>
{
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
pub struct App
{
    exit: bool,
}

impl App
{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>
    {
        while !self.exit
        {
            terminal.draw( |frame: &mut Frame<'_>| { self.draw(frame); })?;
            self.handle_events()?;
        }

        return Ok(());
    }

    fn draw(&self, frame: &mut Frame)
    {
        self.render(frame.area(), frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()>
    {
        match event::read()?
        {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press =>
            {
                self.handle_key_event(key_event)
            }
            _ => { }
        };

        return Ok(());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent)
    {
        match key_event.code
        {
            KeyCode::Esc  => self.exit(),
            KeyCode::Up   => self.up(),
            KeyCode::Down => self.down(),

            _ => { }   
        }
    }

    fn exit(&mut self)
    {
        self.exit = true;
    }

    fn up(&mut self)
    {

    }

    fn down(&mut self)
    {

    }
}

impl Widget for &App
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let title: Line<'_>  = Line::from(" mvtool tui ".bold());
        let frame: Block<'_> = Block::bordered().title(title.right_aligned()).border_set(border::ROUNDED);

        frame.render(area, buf);
    }
}