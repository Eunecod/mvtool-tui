
//   /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$  | 
//  /$$__  $$| $$__  $$ /$$__  $$ /$$__  $$ | [esud] mvtool v0.1.0
// | $$$$$$$$| $$$$$$$/| $$  | $$| $$$$$$$$ | 30/01/2025
// | $$  | $$| $$  | $$|  $$$$$$/| $$  | $$ | 
// |__/  |__/|__/  |__/ \____ $$$|__/  |__/ | Лецензии нет делай все что хочешь форкай не форкай копипасти ломай строй и т.д. :)
//                            \__/          | 

mod objects;
mod ui;

use std::{fs, io, path::Path};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::{DefaultTerminal, Frame, buffer::{Buffer}, layout::{Constraint, Layout, Rect, Spacing}, style::{Color, Style, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, Padding, Paragraph, Widget}};
use serde_json::{Value};

use objects::{Component, Project, Configure}; 
use ui::checkbox::{Checkbox, CheckboxState, LayoutCheckboxGroup, VeticalCheckboxGroup, HorizontalCheckboxGroup};
use ui::messagelog::{MessageLog, MessageType};

fn main() -> io::Result<()>
{
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum ActiveArea
{
    #[default]
    Project, 
    Configure,
    Component,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum DirectionTab
{
    #[default]
    Next,
    Prev,
}

#[derive(Debug, Default)]
struct Setting
{
    path: String,
}

impl Setting
{
    fn new(path: String) -> Self
    {
        return Self { path: path };
    }

    fn get_path(&self) -> &String
    {
        return &self.path;
    }

}

#[derive(Debug, Default)]
pub struct App
{
    projects: Vec<Project>,
    configures: Vec<Configure>,
    setting: Setting,

    selected_project: usize,
    selected_configure: usize,
    selected_component: usize,

    active_area: ActiveArea,

    message_log: MessageLog,

    exit: bool,
}

impl App
{
    pub fn init(&mut self)
    {
        self.active_area = ActiveArea::Project;
        self.selected_project   = 0;
        self.selected_configure = 0;
        self.selected_component = 0;

        self.message_log = MessageLog::default();

        let content = fs::read_to_string("D:/code/mvtool-tui/target/debug/setting.json").unwrap_or(String::new()); 

        let json_body: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
        if json_body.is_null()
        {
            self.message_log.add_message("Initialization mvtool finished error: not load json".to_string(), MessageType::Error);
            return;
        }

        for project in json_body["projects"].as_array().unwrap()
        {
            let mut components: Vec<Component> = Vec::new();
            for component in project["components"].as_array().unwrap()
            {
                components.push(Component::new(component["name"].to_string(), component["selected"].as_bool().unwrap_or(false)));
            }

            self.projects.push(Project::new(project["name"].to_string(), project["path"].to_string(), components, project["selected"].as_bool().unwrap_or(false)));
        }

        for configure in json_body["configure"].as_array().unwrap()
        {
            self.configures.push(Configure::new(configure["name"].to_string(), configure["selected"].as_bool().unwrap_or(false)));
        }

        self.setting = Setting::new(json_body["settings"]["path"].as_str().unwrap_or("").to_string()); 
        if self.setting.get_path().is_empty()
        {
            self.message_log.add_message("Initialization mvtool finished error: setting path is empty".to_string(), MessageType::Error);
            return;
        }    

        self.message_log.add_message("Initialization success done mvtool".to_string(), MessageType::Info);    
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>
    {
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
            KeyCode::Up        => self.up(),
            KeyCode::Down      => self.down(),
            KeyCode::Left      => self.left(),
            KeyCode::Right     => self.right(),
            KeyCode::Char(' ') => self.pick(),
            KeyCode::F(1)      => self.ok(),

            KeyCode::Esc       => self.exit(),
            KeyCode::Tab       => self.toggle_active_area(DirectionTab::Next),
            KeyCode::BackTab   => self.toggle_active_area(DirectionTab::Prev),

            _ => { }   
        }
    }

    fn exit(&mut self)
    {
        self.exit = true;
    }

    fn ok(&mut self)
    {
        // self.message.clear();

        let destination_path: String = self.setting.get_path().replace("{PROJECT}", self.projects[self.selected_project].get_name().trim_matches('"'));
        let destination_dir: &Path = Path::new(&destination_path);

        for component in self.projects[self.selected_project].get_components()
        {
            if self.projects[self.selected_project].is_selected() && component.is_selected()
            {
                let component_name: String = component.get_name().trim_matches('"').to_string();
                let source_file: std::path::PathBuf = Path::new(self.projects[self.selected_project].get_path().trim_matches('"')).join(&component_name);
            
                if !source_file.exists()
                {
                    self.message_log.add_message(format!("Not exists file copy: {}", source_file.display()), MessageType::Warning);
                    return;
                }
                if !destination_dir.exists()
                {
                    self.message_log.add_message(format!("Not exists directory coped: {}", destination_dir.display()), MessageType::Warning);
                    return;
                }
                
                let destination_file: std::path::PathBuf = destination_dir.join(&component_name);

                self.message_log.add_message(format!("Coped {} file to {}", self.projects[self.selected_project].get_components().len(), destination_dir.display()), MessageType::Success);
                fs::copy(&source_file, &destination_file).unwrap();
            }
        }
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

    fn toggle_active_area(&mut self, direction: DirectionTab)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                if direction == DirectionTab::Next
                {
                    self.active_area = ActiveArea::Configure;
                }
                else
                {
                    self.active_area = ActiveArea::Component;
                }
            },
            ActiveArea::Configure => 
            {
                if direction == DirectionTab::Next
                {
                    self.active_area = ActiveArea::Component;
                }
                else
                {
                    self.active_area = ActiveArea::Project;
                }
            },
            ActiveArea::Component => 
            {
                if direction == DirectionTab::Next
                {
                    self.active_area = ActiveArea::Project;
                }
                else 
                {
                    self.active_area = ActiveArea::Configure;
                }
            },   
        };
    }

    fn pick(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                for project in self.projects.iter_mut()
                {
                    project.set_selected(false);
                }
                
                let is_selected: bool = self.projects[self.selected_project].is_selected();
                self.projects[self.selected_project].set_selected(!is_selected);
            },
            ActiveArea::Configure => 
            {
                for configure in self.configures.iter_mut()
                {
                    configure.set_selected(false);
                }

                let is_selected: bool = self.configures[self.selected_configure].is_selected();
                self.configures[self.selected_configure].set_selected(!is_selected);
            },
            ActiveArea::Component => 
            {
                let is_selected: bool = self.projects[self.selected_project].get_components()[self.selected_component].is_selected();
                self.projects[self.selected_project].get_components_mut()[self.selected_component].set_selected(!is_selected);
            }, 
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

        let mut projects_horizontal_checkbox_group: HorizontalCheckboxGroup = HorizontalCheckboxGroup::new();
        projects_horizontal_checkbox_group.aligment(ratatui::layout::Alignment::Center);
        for (index, project) in self.projects.iter().map(|project: &Project| project.get_name()).enumerate()
        {
            let mut checkbox: Checkbox<'_> = Checkbox::new(project);
            let mut state: CheckboxState = CheckboxState::new(self.projects[index].is_selected()); 
            state.set_enable_signed_highlight(false);

            match self.active_area
            {
                ActiveArea::Project =>
                {
                    state.focus();
                    if self.selected_project == index
                    {
                        state.highlight();
                    }
                }
                _ => { }
            }

            checkbox.set_state(state);
            projects_horizontal_checkbox_group.add_checkbox(checkbox);
        }

        // CONFIGURES LIST
        let configures_block: Block<'_> = Block::bordered().title(" configure ".bold()).border_set(border::THICK).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 })
        .border_style(if self.active_area == ActiveArea::Configure { Style::default().fg(Color::Gray) } else { Style::default().fg(Color::DarkGray) });

        let mut configures_vetical_checkbox_group: VeticalCheckboxGroup = VeticalCheckboxGroup::new();
        configures_vetical_checkbox_group.aligment(ratatui::layout::Alignment::Left);
        for (index,configure) in self.configures.iter().enumerate()
        {
            let mut checkbox: Checkbox<'_> = Checkbox::new(&configure.get_name());
            let mut state: CheckboxState = CheckboxState::new(configure.is_selected()); 

            match self.active_area
            {
                ActiveArea::Configure =>
                {
                    state.focus();
                    if self.selected_configure == index
                    {
                        state.highlight();
                    }
                }
                _ => { }
            }

            checkbox.set_state(state);
            configures_vetical_checkbox_group.add_checkbox(checkbox);
        }

        // COMPONENTS LIST  
        let components_block: Block<'_> = Block::bordered().title(" components ".bold()).border_set(border::THICK).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 })
        .border_style(if self.active_area == ActiveArea::Component { Style::default().fg(Color::Gray) } else { Style::default().fg(Color::DarkGray) });

        let mut components_vetical_checkbox_group: VeticalCheckboxGroup = VeticalCheckboxGroup::new();
        components_vetical_checkbox_group.aligment(ratatui::layout::Alignment::Left);
        for (index, component) in self.projects[self.selected_project].get_components().iter().enumerate()
        {
            let mut checkbox: Checkbox<'_> = Checkbox::new(&component.get_name());
            let mut state: CheckboxState = CheckboxState::new(component.is_selected()); 

            match self.active_area
            {
                ActiveArea::Component =>
                {
                    state.focus();
                    if self.selected_component == index
                    {
                        state.highlight();
                    }
                }
                _ => { }
            }

            checkbox.set_state(state);
            components_vetical_checkbox_group.add_checkbox(checkbox);
        }

        // CONSOLE
        let console_block: Block<'_> = Block::bordered().title(" console ").border_set(border::THICK).padding(Padding { left: 1, right: 0, top: 0, bottom: 0 });
        let console: Paragraph<'_> = self.message_log.get_message().block(console_block.style(Color::Gray));

        // RENDER MAINLAYOUT 0
        logo_block.render(logo_area, buf);

        // RENDER SUBLAYOUT 0
        projects_horizontal_checkbox_group.render(projects_block.inner(project_area), buf);
        projects_block.render(project_area, buf);

        // RENDER SUBLAYOUT 1
        components_vetical_checkbox_group.render(components_block.inner(components_area), buf);
        components_block.render(components_area, buf);

        configures_vetical_checkbox_group.render(configures_block.inner(configures_area), buf);
        configures_block.render(configures_area, buf);

        // RENDER MAINLAYOUT 1
        console.render(console_area, buf);
    }
}