
//   /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$  | 
//  /$$__  $$| $$__  $$ /$$__  $$ /$$__  $$ | [esud] mvtool v0.1.0
// | $$$$$$$$| $$$$$$$/| $$  | $$| $$$$$$$$ | 30/01/2025
// | $$  | $$| $$  | $$|  $$$$$$/| $$  | $$ | 
// |__/  |__/|__/  |__/ \____ $$$|__/  |__/ | Лецензии нет делай все что хочешь форкай не форкай копипасти ломай строй и т.д. :)
//                            \__/          | 

use std::{io, fs};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect, Spacing}, style::{Color, Style, Stylize}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Padding, Paragraph, Widget}};
use serde_json::{Value};

fn main() -> io::Result<()>
{
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
pub struct Component
{
    pub name: String,
    pub selected: bool,
}

impl Component
{
    fn new(name: String, selected: bool) -> Self
    {
        return Self { name: name, selected: selected };
    }

    fn get_name(&self) -> &String
    {
        return &self.name;
    }

    fn is_selected(&self) -> bool
    {
        return self.selected;
    }

    fn set_selected(&mut self, selected: bool)
    {
        self.selected = selected;
    }
}

#[derive(Debug, Default)]
pub struct Project
{
    pub name: String,
    pub path: String,
    pub components: Vec<Component>,
    pub selected: bool,
}

impl Project
{
    fn new(name: String, path: String, components: Vec<Component>, selected: bool) -> Self
    {
        return Self { name: name, path: path, components: components, selected: selected };
    }

    fn get_name(&self) -> &String
    {
        return &self.name;
    }

    fn get_path(&self) -> &String
    {
        return &self.path;
    }

    fn get_components(&self) -> &Vec<Component>
    {
        return &self.components;
    }

    fn is_selected(&self) -> bool
    {
        return self.selected;
    }

    fn set_selected(&mut self, selected: bool)
    {
        self.selected = selected;
    }
}

#[derive(Debug, Default)]
pub struct Configure
{
    pub name: String,
    pub selected: bool,
}

impl Configure
{
    fn new(name: String, selected: bool) -> Self
    {
        return Self { name: name, selected: selected };
    }

    fn get_name(&self) -> &String
    {
        return &self.name;
    }

    fn is_selected(&self) -> bool
    {
        return self.selected;
    }

    fn set_selected(&mut self, selected: bool)
    {
        self.selected = selected;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ActiveArea
{
    #[default]
    Project, 
    Configure,
    Component,
}

#[derive(Debug, Default)]
pub struct App
{
    projects: Vec<Project>,
    configures: Vec<Configure>,

    selected_project: usize,
    selected_configure: usize,
    selected_component: usize,

    active_area: ActiveArea,

    exit: bool,
}

impl App
{
    pub fn init(&mut self)
    {
        let content = fs::read_to_string("D:/code/mvtool-tui/target/debug/setting.json").unwrap(); 
        let json_body: Value = serde_json::from_str(&content).unwrap();

        for (name, value) in json_body["projects"].as_object().unwrap()
        {
            let mut components: Vec<Component> = Vec::new();
            for component in value["components"].as_array().unwrap()
            {
                components.push(Component::new(component["name"].to_string(), component["selected"].as_bool().unwrap_or(false)));
            }

            self.projects.push(Project::new(name.to_string(), value["path"].to_string(), components, value["selected"].as_bool().unwrap_or(false)));
        }

        for config in json_body["configure"].as_array().unwrap()
        {
            self.configures.push(Configure::new(config["name"].to_string(), config["selected"].as_bool().unwrap_or(false)));
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>
    {
        self.selected_project = 0;
        self.selected_configure = 0;
        self.selected_component = 0;
        self.active_area = ActiveArea::Project;

        self.init();

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
            Event::Key(key_event) =>
            {
                if key_event.kind == KeyEventKind::Press
                {
                    self.handle_key_event(key_event)
                }
            }
            _ => { }
        };

        return Ok(());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent)
    {
        match key_event.code
        {
            KeyCode::Esc       => self.exit(),
            KeyCode::Up        => self.up(),
            KeyCode::Down      => self.down(),
            KeyCode::Left      => self.left(),
            KeyCode::Right     => self.right(),
            KeyCode::Tab       => self.toggle_active_area(),
            KeyCode::Char(' ') => self.pick(),

            _ => { }   
        }
    }

    fn exit(&mut self)
    {
        self.exit = true;
    }

    fn up(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Configure => 
            {
                if self.selected_configure != 0
                {
                    self.selected_configure -= 1;
                }
            },
            ActiveArea::Component => 
            {
                if self.selected_component != 0
                {
                    self.selected_component -= 1;
                }
            },

            _ => { }   
        };
    }

    fn down(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Component => 
            {
                if self.selected_component < self.projects[self.selected_project].get_components().len() - 1
                {
                    self.selected_component += 1;
                }
            },
            ActiveArea::Configure => 
            {
                if self.selected_configure < self.configures.len() - 1
                {
                    self.selected_configure += 1;
                }
            },

            _ => { }   
        };
    }

    fn left(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Project => 
            {
                 if self.selected_project != 0
                {
                    self.selected_project -= 1;
                }
            },

            _ => { }   
        };
    }

    fn right(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Project => 
            {
                if self.selected_project < self.projects.len() - 1
                {
                    self.selected_project += 1;
                }
            },

            _ => { }   
        };
    }

    fn toggle_active_area(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                self.active_area = ActiveArea::Configure;
            },
            ActiveArea::Configure => 
            {
                self.active_area = ActiveArea::Component;
            },
            ActiveArea::Component => 
            {
                self.active_area = ActiveArea::Project;
            },

            _ => { }   
        };
    }

    fn pick(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                let is_selected: bool = self.projects[self.selected_project].is_selected();
                self.projects[self.selected_project].set_selected(!is_selected);
            },
            ActiveArea::Configure => 
            {
                let is_selected: bool = self.configures[self.selected_configure].is_selected();
                self.configures[self.selected_configure].set_selected(!is_selected);
            },
            ActiveArea::Component => 
            {
                let is_selected: bool = self.projects[self.selected_project].get_components()[self.selected_component].is_selected();
                self.projects[self.selected_project].components[self.selected_component].set_selected(!is_selected);
            },

            _ => { }   
        };
    }
}

impl Widget for &App
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let [logo_area, project_area, setting_area, console_area] = Layout::vertical([
            Constraint::Length(7),
            Constraint::Length(5),
            Constraint::Min(1),
            Constraint::Length(3),
            ])
        .margin(0)
        .spacing(Spacing::Overlap(0))
        .areas(area);
        
        let [configures_area, components_area] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
            ])
        .margin(0)
        .spacing(Spacing::Overlap(0))
        .areas(setting_area);    

        // LOGO
        let logo: Text<'_> = Text::from(vec![
            Line::from("  /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$ "  ).style(Style::default().fg(Color::Blue)),
            Line::from(" /$$__  $$| $$__  $$ /$$__  $$ /$$__  $$"  ).style(Style::default().fg(Color::Blue)),
            Line::from("| $$$$$$$$| $$$$$$$/| $$  | $$| $$$$$$$$"  ).style(Style::default().fg(Color::Blue)),
            Line::from("| $$  | $$| $$  | $$|  $$$$$$/| $$  | $$"  ).style(Style::default().fg(Color::Blue)),
            Line::from("|__/  |__/|__/  |__/ \\____ $$$|__/  |__/" ).style(Style::default().fg(Color::Blue)),
            Line::from("                           \\__/          ").style(Style::default().fg(Color::Blue)),
            Line::from(" [esud] mvtool v0.1.0 ").style(Style::default().fg(Color::DarkGray)).alignment(ratatui::layout::Alignment::Right),
        ]);

        let logo_block: Paragraph<'_> = Paragraph::new(logo).alignment(ratatui::layout::Alignment::Center);

        // PROJECTS LIST
        let projects_block: Block<'_> = Block::bordered().title(" projects ".bold()).border_set(border::THICK).padding(Padding { left: 2, right: 2, top: 1, bottom: 0 })
        .border_style(if self.active_area == ActiveArea::Project { Style::default().fg(Color::Gray) } else { Style::default().fg(Color::DarkGray) });

        let mut projects_spans: Vec<Span<'_>> = Vec::new();
        for (index, project) in self.projects.iter().map(|project: &Project| project.get_name()).enumerate()
        {   
            let style: Style;
            match self.active_area
            {
                ActiveArea::Project =>
                {
                    style = if index == self.selected_project { Style::default().fg(Color::Green).bold() } else { Style::default().fg(Color::White) };
                }
                _ =>
                { 
                    style = Style::default().fg(Color::DarkGray);
                }
            }
            
            let pick: char = if self.projects[index].is_selected() { 'x' } else { ' ' }; 

            projects_spans.push(Span::styled(format!(" [{}] {}", pick, project), style));
        }

        let projects_line: Line<'_> = Line::from(projects_spans);
        let projects_list: Paragraph<'_> = Paragraph::new(projects_line).block(projects_block).alignment(ratatui::layout::Alignment::Center);
    
        // CONFIGURES LIST
        let configures_block: Block<'_> = Block::bordered().title(" configure ".bold()).border_set(border::THICK).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 })
        .border_style(if self.active_area == ActiveArea::Configure { Style::default().fg(Color::Gray) } else { Style::default().fg(Color::DarkGray) });

        let mut configures_lines: Vec<Line<'_>> = Vec::new();
        for (index, configure) in self.configures.iter().enumerate()
        {   
            let style: Style;
            let highlited: char;
            match self.active_area
            {
                ActiveArea::Configure =>
                {
                    style = if index == self.selected_configure { Style::default().fg(Color::Green).bold() } else { Style::default().fg(Color::White) };
                    highlited = if index == self.selected_configure { '>' } else { ' ' };
                }
                _ =>
                { 
                    style = Style::default().fg(Color::DarkGray);
                    highlited = ' ';
                }
            }

            let pick: char = if configure.is_selected() { 'x' } else { ' ' }; 

            configures_lines.push(Line::from(Span::styled(format!(" {} [{}] {}", highlited, pick, configure.get_name()), style)));
        }

        let configures_list: Paragraph<'_> = Paragraph::new(Text::from(configures_lines)).block(configures_block).alignment(ratatui::layout::Alignment::Left);
            
        // COMPONENTS LIST
        let components_block: Block<'_> = Block::bordered().title(" components ".bold()).border_set(border::THICK).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 })
        .border_style(if self.active_area == ActiveArea::Component { Style::default().fg(Color::Gray) } else { Style::default().fg(Color::DarkGray) });

        let mut components_lines: Vec<Line<'_>> = Vec::new();
        for (index, component) in self.projects[self.selected_project].get_components().iter().enumerate()
        {   
            let style: Style;
            let highlited: char;
            match self.active_area
            {
                ActiveArea::Component =>
                {
                    style = if index == self.selected_component { Style::default().fg(Color::Green).bold() } else { Style::default().fg(Color::White) };
                    highlited = if index == self.selected_component { '>' } else { ' ' };
                }
                _ =>
                { 
                    style = Style::default().fg(Color::DarkGray);
                    highlited = ' ';
                }
            }

            let pick: char = if component.is_selected() { 'x' } else { ' ' }; 

            components_lines.push(Line::from(Span::styled(format!(" {} [{}] {}", highlited, pick, component.get_name()), style)));
        }

        let components_list: Paragraph<'_> = Paragraph::new(Text::from(components_lines)).block(components_block).alignment(ratatui::layout::Alignment::Left);

        // CONSOLE
        let console_block: Block<'_> = Block::bordered().title(" console ").border_set(border::THICK).padding(Padding { left: 2, right: 0, top: 0, bottom: 0 });
        
        let message = Text::from("Applying settings...").style(Style::default().fg(Color::Green));
        let console_paragraph: Paragraph<'_> = Paragraph::new(message).block(console_block);

        // RENDER MAINLAYOUT 0
        logo_block.render(logo_area, buf);

        // RENDER SUBLAYOUT 0
        projects_list.render(project_area, buf);

        // RENDER SUBLAYOUT 1
        components_list.render(components_area, buf);
        configures_list.render(configures_area, buf);

        // RENDER MAINLAYOUT 1
        console_paragraph.render(console_area, buf);
    }
}