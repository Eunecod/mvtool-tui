// src/main.rs

//   /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$  | 
//  /$$__  $$| $$__  $$ /$$__  $$ /$$__  $$ | [esud] mvtool v1.1.3
// | $$$$$$$$| $$$$$$$/| $$  | $$| $$$$$$$$ | 30/01/2025
// | $$  | $$| $$  | $$|  $$$$$$/| $$  | $$ | 
// |__/  |__/|__/  |__/ \____ $$$|__/  |__/ | Лецензии нет делай все что хочешь форкай не форкай копипасти ломай строй и т.д. :)
//                            \__/          |   

mod objects;
mod utils;
mod ui;

use std::{ fs, io, process::Command };
use crossterm::event::{ self, Event, KeyCode, KeyEventKind };
use ratatui::{ buffer::Buffer, symbols::border, layout::{ Constraint, Layout, Rect }, style::{ Color, Style, Stylize, Modifier }, text::{ Line }, widgets::{ Block, Paragraph, Widget, Padding, StatefulWidget }, DefaultTerminal, Frame };
use serde_json::Value;
use ui::checkbox::layout::LayoutCheckboxGroup;
use ui::checkbox::{ Checkbox, CheckboxState, HorizontalCheckboxGroup, VerticalCheckboxGroup, CheckboxGroupState };
use ui::messagelog::{ MessageLog, MessageType, LogEvent };
use objects::{ Project, Script, Configure };
use utils::Utils;

fn main() -> io::Result<()>
{
    return ratatui::run(|terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<io::Stdout>>| App::default().run(terminal));
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum ActiveArea
{
    #[default] Project,
    Configure,
    Component,
    Scripts,
}

#[derive(Debug)]
pub struct App
{
    projects: Vec<Project>,
    extension_mask: Vec<String>,
    selected_project: usize,
    selected_configure: usize,
    selected_component: usize,
    selected_script: usize,
    active_area: ActiveArea,
    message_log: MessageLog,
    exit: bool,
}

impl Default for App
{
    fn default() -> Self
    {
        return Self
        {
            projects: Vec::new(),
            extension_mask: Vec::new(),
            selected_project: 0,
            selected_configure: 0,
            selected_component: 0,
            selected_script: 0,
            active_area: ActiveArea::Project,
            message_log: MessageLog::new(),
            exit: false
        };
    }
}

impl App
{
    pub fn init(&mut self)
    {
        let content: String = fs::read_to_string("setting.json").unwrap_or_default();
        let json: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
        if json.is_null()
        {
            self.message_log.add_message("failed to load setting.json".into(), MessageType::Error);
            return;
        }

        self.extension_mask = json["extension_mask"].as_array().unwrap_or(&vec![]).iter().filter_map(|v| v.as_str()).map(|s| s.replace("*.", "")) .collect();

        for projects in json["projects"].as_array().unwrap_or(&vec![])
        {
            let mut configures: Vec<Configure> = Vec::new();
            for configure in projects["configures"].as_array().unwrap_or(&vec![])
            {
                let components: Vec<objects::Component> = configure["components"].as_array().unwrap_or(&vec![]).iter().map(|component: &Value|
                    {
                        objects::Component::new(component["name"].as_str().unwrap_or_default().to_string(), component["selected"].as_bool().unwrap_or(false))
                    }
                ).collect();

                let scripts: Vec<Script> = configure["scripts"].as_array().unwrap_or(&vec![]).iter().map(|script|
                    {
                        Script::new(script["name"].as_str().unwrap_or_default().to_string(), script["command"].as_str().unwrap_or_default().to_string())
                    }
                ).collect();

                configures.push(Configure::new(
                    configure["name"].as_str().unwrap_or_default().to_string(),
                    configure["source_path"].as_str().unwrap_or_default().to_string(),
                    configure["selected"].as_bool().unwrap_or(false),
                    configure["clean_destination"].as_bool().unwrap_or(false),
                    components, scripts,
                ));
            }
            self.projects.push(Project::new(
                projects["name"].as_str().unwrap_or_default().to_string(),
                projects["destination_path"].as_str().unwrap_or_default().to_string(),
                configures, projects["selected"].as_bool().unwrap_or(false),
            ));
        }

        self.message_log.add_message("initialized 'mvtool'".into(), MessageType::Success);
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>
    {
        self.init();
        while !self.exit
        {
            terminal.draw(|f: &mut Frame<'_>| self.draw(f))?;

            self.message_log.update();

            if event::poll(std::time::Duration::from_millis(16))?
            {
                self.handle_events()?;
            }
        }

        return Ok(());
    }

    fn draw(&self, frame: &mut Frame)
    {
        self.render(frame.area(), frame.buffer_mut());
    }

    fn area_style(&self, area: ActiveArea) -> Style
    {
        if self.active_area == area
        {
            return Style::default().fg(Color::White);
        }
        else
        {
            return Style::default().fg(Color::DarkGray);
        }
    }

    fn handle_events(&mut self) -> io::Result<()>
    {
        if let Event::Key(key) = event::read()?
        {
            if key.kind == KeyEventKind::Press
            {
                match key.code
                {
                    KeyCode::Up        => self.move_selection(-1),
                    KeyCode::Down      => self.move_selection(1),
                    KeyCode::Left      => self.move_project(-1),
                    KeyCode::Right     => self.move_project(1),
                    KeyCode::Char(' ') => self.on_action(),
                    KeyCode::Tab       => self.next_area(true),
                    KeyCode::BackTab   => self.next_area(false),
                    KeyCode::F(1)      => self.ok(),
                    KeyCode::Esc       => self.exit = true,
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    fn move_selection(&mut self, delta: i32)
    {
        match self.active_area
        {
            ActiveArea::Configure =>
            {
                let len: usize = self.projects[self.selected_project].get_configures().len();
                if len == 0
                {
                    return;
                }

                self.selected_configure = (self.selected_configure as i32 + delta).clamp(0, len as i32 - 1) as usize;
            }
            ActiveArea::Component =>
            {
                let len: usize = self.projects[self.selected_project].get_configures()[self.selected_configure].get_components().len();
                if len == 0
                {
                    return;
                }

                self.selected_component = (self.selected_component as i32 + delta).clamp(0, len as i32 - 1) as usize;
            }
            ActiveArea::Scripts =>
            {
                let len: usize = self.projects[self.selected_project].get_configures()[self.selected_configure].get_scripts().len();
                if len == 0
                {
                    return;
                }

                self.selected_script = (self.selected_script as i32 + delta).clamp(0, len as i32 - 1) as usize;
            }

            _ => {}
        }
    }

    fn move_project(&mut self, delta: i32)
    {
        if self.active_area == ActiveArea::Project && !self.projects.is_empty()
        {
            self.selected_project = (self.selected_project as i32 + delta).clamp(0, self.projects.len() as i32 - 1) as usize;
            self.selected_configure = 0;
        }
    }

    fn get_selected_current_project(&self) -> usize
    {
        for (i, project) in self.projects.iter().enumerate()
        {
            if project.is_selected()
            {
                return i;
            }
        }

        return 0 as usize;
    }

    fn get_selected_current_configure(&self) -> usize
    {
        for (i, configure) in self.projects[self.selected_project].get_configures().iter().enumerate()
        {
            if configure.is_selected()
            {
                return i;
            }
        }

        return 0 as usize;
    }

    fn next_area(&mut self, forward: bool)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                self.selected_project = self.get_selected_current_project();
            }
            ActiveArea::Configure =>
            {
                self.selected_configure = self.get_selected_current_configure();
            }

            _ => {}
        }

        self.active_area = match (self.active_area, forward)
        {
            (ActiveArea::Project, true)    => ActiveArea::Configure,
            (ActiveArea::Configure, true)  => ActiveArea::Component,
            (ActiveArea::Component, true)  => ActiveArea::Scripts,
            (ActiveArea::Scripts, true)    => ActiveArea::Project,
            (ActiveArea::Project, false)   => ActiveArea::Scripts,
            (ActiveArea::Scripts, false)   => ActiveArea::Component,
            (ActiveArea::Component, false) => ActiveArea::Configure,
            (ActiveArea::Configure, false) => ActiveArea::Project,
        };
    }

    fn on_action(&mut self)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                if self.projects.is_empty()
                {
                    return;
                }

                let selected: bool = self.projects[self.selected_project].is_selected();
                self.projects.iter_mut().for_each(|p: &mut Project| p.set_selected(false));
                self.projects[self.selected_project].set_selected(!selected);
            }
            ActiveArea::Configure =>
            {
                if self.projects[self.selected_project].get_configures().is_empty()
                {
                    return;
                }

                let project: &mut Project = &mut self.projects[self.selected_project];
                let selected: bool = project.get_configures()[self.selected_configure].is_selected();

                project.get_configures_mut().iter_mut().for_each(|configure: &mut Configure| configure.set_selected(false));
                project.get_configures_mut()[self.selected_configure].set_selected(!selected);
            }
            ActiveArea::Component =>
            {
                if self.projects[self.selected_project].get_configures()[self.selected_configure].get_components().is_empty()
                {
                    return;
                }

                let component: &mut objects::Component = &mut self.projects[self.selected_project].get_configures_mut()[self.selected_configure].get_components_mut()[self.selected_component];
                component.set_selected(!component.is_selected());
            }
            ActiveArea::Scripts =>
            {
                if self.projects[self.selected_project].get_configures()[self.selected_configure].get_scripts().is_empty()
                {
                    return;
                }

                let script: &Script = &self.projects[self.selected_project].get_configures()[self.selected_configure].get_scripts()[self.selected_script];
                let config: &Configure = &self.projects[self.selected_project].get_configures()[self.selected_configure];

                self.message_log.add_message(format!("running: {}", script.get_name()), MessageType::Info);

                let status: Result<std::process::ExitStatus, io::Error> = Command::new("cmd").args(
                    ["/C", "start", format!("mvtool: {}", script.get_name()).as_str(), "cmd", "/K", script.get_command()]).current_dir(config.get_path()
                ).status();

                match status
                {
                    Ok(stat)
                    if stat.success() =>
                    {
                        self.message_log.add_message(format!("executed script: {}", script.get_name()), MessageType::Success);
                    }
                    _ =>
                    {
                        self.message_log.add_message(format!("failed to run or script error: {}", script.get_name()), MessageType::Error);
                    }
                }
            }
        }
    }

    fn ok(&mut self)
    {
        let projects: Vec<Project> = self.projects.clone();
        let extension_mask: Vec<String> = self.extension_mask.clone();
        let tx: std::sync::mpsc::Sender<ui::messagelog::LogEvent> = self.message_log.get_sender();

        self.message_log.add_message("starting...".into(), MessageType::Info);
        std::thread::spawn(move || {
            let mut fallback: bool = true;
            for project in projects
            {
                if !project.is_selected()
                {
                    continue;
                }
                let dest_path: &String = project.get_destination();

                for configure in project.get_configures()
                {
                    if !configure.is_selected()
                    {
                        continue;
                    }
                    let src_path: &String = configure.get_path();

                    fallback = false;

                    if configure.should_clean()
                    { 
                        tx.send(LogEvent { message: "cleaning components...".into(), message_type: MessageType::Info }).unwrap_or_default();

                        if let Ok(entries) = fs::read_dir(dest_path)
                        {
                            for entry in entries.flatten()
                            {
                                let file_name: String = entry.file_name().to_string_lossy().to_string();
                                
                                let matches_mask: bool = extension_mask.is_empty() || extension_mask.iter().any(|ext: &String| file_name.ends_with(ext));
                                if !matches_mask
                                {
                                    continue;
                                }

                                for component in configure.get_components()
                                {
                                    if Utils::is_match(&entry.path(), component.get_name(), &extension_mask)
                                    {
                                        let _ = fs::remove_file(entry.path());
                                        break; 
                                    }
                                }
                            }
                        }
                    }

                    let mut copied_count: usize = 0;
                    let mut copied_total: usize = 0;
                    if let Ok(entries) = fs::read_dir(src_path)
                    {
                        for entry in entries.flatten()
                        {
                            let file_name: String = entry.file_name().to_string_lossy().to_string();

                            let matches_mask: bool = extension_mask.is_empty() || extension_mask.iter().any(|ext: &String| file_name.ends_with(ext));
                            if !matches_mask
                            {
                                continue;
                            }

                            for component in configure.get_components()
                            {
                                if !component.is_selected()
                                {
                                    continue;
                                }

                                if Utils::is_match(&entry.path(), component.get_name(), &extension_mask)
                                {
                                    let to: std::path::PathBuf = std::path::Path::new(dest_path).join(&file_name);
                                    if fs::copy(entry.path(), to).is_ok()
                                    {
                                        copied_count += 1;
                                        tx.send(LogEvent { message: format!("processed copying {}: {}", copied_count, file_name), message_type: MessageType::Info }).unwrap_or_default();
                                    }
                                    copied_total += 1;
                                }
                            }
                        }

                        tx.send(LogEvent { message: format!("finished copying {}/{}: {}", copied_count, copied_total, dest_path), message_type: MessageType::Success }).unwrap_or_default();
                    }
                    else
                    {
                        tx.send(LogEvent { message: format!("failed to read source directory: {}", src_path), message_type: MessageType::Warning }).unwrap_or_default();
                    }
                }
            }

            if fallback
            {
                tx.send(LogEvent { message: "no project or configure selected, nothing was done".into(), message_type: MessageType::Warning }).unwrap_or_default();
            }
        });
    }
}

impl Widget for &App
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let [logo_area, project_area, middle_area, console_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Min(1),
            Constraint::Length(5),
        ]).areas(area);

        let [side_area, component_area] = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).areas(middle_area);
        let [configure_area, script_area] = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(side_area);

        // LOGO
        let logo_lines: Vec<Line<'_>> = vec![
            Line::from(" [esud] mvtool v1.1.3 ").dark_gray().right_aligned(),
        ];
        let logo_block: Paragraph<'_> = Paragraph::new(logo_lines).alignment(ratatui::layout::Alignment::Center);

        // PROJECT LIST
        let mut project_group: HorizontalCheckboxGroup<'_> = HorizontalCheckboxGroup::new();
        let project_block: Block<'_> = Block::bordered().title("[ projects ]".bold()).border_set(border::ROUNDED).border_style(self.area_style(ActiveArea::Project)).padding(Padding { left: 2, right: 2, top: 1, bottom: 0 });

        // CONFIGURE LIST
        let mut configure_group: VerticalCheckboxGroup<'_> = VerticalCheckboxGroup::new();
        let configure_block: Block<'_> = Block::bordered().title("[ configure ]").border_set(border::ROUNDED).border_style(self.area_style(ActiveArea::Configure)).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 });

        // COMPONENT LIST
        let mut component_group: VerticalCheckboxGroup<'_> = VerticalCheckboxGroup::new();
        let component_block: Block<'_> = Block::bordered().title("[ components ]").border_set(border::ROUNDED).border_style(self.area_style(ActiveArea::Component)).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 });
        
        // SCRIPT LIST
        let mut script_group: VerticalCheckboxGroup<'_> = VerticalCheckboxGroup::new();
        let script_block: Block<'_> = Block::bordered().title("[ scripts ]").border_set(border::ROUNDED).border_style(self.area_style(ActiveArea::Scripts)).padding(Padding { left: 2, right: 2, top: 1, bottom: 1 });

        // CONSOLE
        let console_block: Block<'_> = Block::bordered().title("[ console ]").border_set(border::ROUNDED).padding(Padding { left: 1, right: 0, top: 1, bottom: 1 });

        if !self.projects.is_empty()
        {
            // PROJECT LIST
            for (i, p) in self.projects.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(p.is_selected());
                if self.active_area == ActiveArea::Project
                {
                    state.focus();
                    if i == self.selected_project
                    {
                        state.highlight();
                    }
                }  

                project_group.add_checkbox(
                    {
                        let mut checkbox: Checkbox<'_> = Checkbox::new(p.get_name());
                        checkbox.set_state(state);
                        checkbox
                    }
                );
            }

            // CONFIGURE LIST
            let configures: &Vec<Configure> = self.projects[self.selected_project].get_configures();
            for (i, configure) in configures.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(configure.is_selected());
                if self.active_area == ActiveArea::Configure
                {
                    state.focus(); if i == self.selected_configure
                    {
                        state.highlight();
                    }
                }

                configure_group.add_checkbox(
                    {
                        let mut checkbox: Checkbox<'_> = Checkbox::new(configure.get_name());
                        checkbox.set_state(state);
                        checkbox
                    }
                );
            }

            // COMPONENT LIST
            let components: &Vec<objects::Component> = configures[self.selected_configure].get_components();
            for (i, component) in components.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(component.is_selected());
                if self.active_area == ActiveArea::Component
                {
                    state.focus();
                    if i == self.selected_component
                    {
                        state.highlight();
                    }
                }

                component_group.add_checkbox(
                    {
                        let mut checkbox: Checkbox<'_> = Checkbox::new(component.get_name());
                        checkbox.set_state(state);
                        checkbox
                    }
                );
            }

            // SCRIPT LIST
            let scripts: &Vec<Script> = configures[self.selected_configure].get_scripts();
            for (i, script) in scripts.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(false);
                state.data.symbols = Some(("", ">"));
                state.data.style_highlighted = Some(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

                if self.active_area == ActiveArea::Scripts
                {
                    state.focus();
                    if i == self.selected_script
                    {
                        state.data.is_selected = true;
                        state.highlight();
                    }
                }

                script_group.add_checkbox(
                    {
                        let mut checkbox: Checkbox<'_> = Checkbox::new(script.get_name());
                        checkbox.set_state(state);
                        checkbox
                    }
                );
            }
        }

        // CONSOLE
        let console: Paragraph<'_> = self.message_log.get_message().block(console_block.style(Color::Gray));
        
        // RENDER MAINLAYOUT 0
        logo_block.render(logo_area, buf);

        // RENDER SUBLAYOUT 0
        project_group.render(project_block.inner(project_area), buf, &mut CheckboxGroupState { cursor: self.selected_project, scroll_offset: 0 });
        project_block.render(project_area, buf);

        // RENDER SUBLAYOUT 1
        component_group.render(component_block.inner(component_area), buf, &mut CheckboxGroupState { cursor: self.selected_component, scroll_offset: 0 });
        component_block.render(component_area, buf);

        // RENDER SUBLAYOUT 2
        configure_group.render(configure_block.inner(configure_area), buf, &mut CheckboxGroupState { cursor: self.selected_configure, scroll_offset: 0 });
        configure_block.render(configure_area, buf);

        // RENDER SUBLAYOUT 3
        script_group.render(script_block.inner(script_area), buf, &mut CheckboxGroupState { cursor: self.selected_script, scroll_offset: 0 });
        script_block.render(script_area, buf);

        // RENDER MAINLAYOUT 1
        console.render(console_area, buf);
    }
}