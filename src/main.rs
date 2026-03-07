// src/main.rs

//   /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$  | 
//  /$$__  $$| $$__  $$ /$$__  $$ /$$__  $$ | [esud] mvtool v1.3.0
// | $$$$$$$$| $$$$$$$/| $$  | $$| $$$$$$$$ | 30/01/2025
// | $$  | $$| $$  | $$|  $$$$$$/| $$  | $$ | 
// |__/  |__/|__/  |__/ \____ $$$|__/  |__/ | Лецензии нет делай все что хочешь форкай не форкай копипасти ломай строй и т.д. :)
//                            \__/          |   

mod objects;
mod utils;
mod ui;
mod updater;

use std::{ fs, io, ops::Add, process::Command };
use crossterm::event::{ self, Event, KeyCode, KeyEventKind };
use ratatui::{ DefaultTerminal, Frame, buffer::Buffer, layout::{ Alignment, Constraint, Layout, Rect }, style::{ Color, Modifier, Style, Stylize }, symbols::border, text::Line, widgets::{ Block, Padding, Paragraph, StatefulWidget, Widget } };
use serde_json::Value;
use ui::checkbox::layout::LayoutCheckboxGroup;
use ui::checkbox::{ Checkbox, CheckboxState, HorizontalCheckboxGroup, VerticalCheckboxGroup, CheckboxGroupState };
use ui::messagelog::{ MessageLog, MessageType };
use ui::messagebox::MessageBox;
use ui::spin::{ Spin, SpinState };
use objects::{ Project, Script, Configure };
use utils::Utils;
use updater::{ Updater, ReleaseUpdateGithub };

use std::sync::mpsc;

fn main() -> io::Result<()>
{
    return ratatui::run(|terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<io::Stdout>>| App::default().run(terminal));
}

#[derive(Clone, Copy, PartialEq, Default)]
enum ActiveArea
{
    #[default] Project,
    Configure,
    Component,
    Scripts,
}

pub enum AppEvent
{
    UpdateAvailable(ReleaseUpdateGithub),
    Log(String, MessageType),
    WaitProcess(SpinState),
}

pub struct App
{
    event_bus: (mpsc::Sender<AppEvent>, mpsc::Receiver<AppEvent>),
    
    message_box: Option<MessageBox<'static>>,
    projects: Vec<Project>,
    state_project: CheckboxGroupState,
    state_configure: CheckboxGroupState,
    state_component: CheckboxGroupState,
    state_script: CheckboxGroupState,
    active_area: ActiveArea,
    message_log: MessageLog,
    spin: Spin,
    exit: bool,
}

impl Default for App
{
    fn default() -> Self
    {
        return Self
        {
            event_bus: std::sync::mpsc::channel(),
            
            message_box: None,
            projects: Vec::new(),
            state_project: CheckboxGroupState::default(),
            state_configure: CheckboxGroupState::default(),
            state_component: CheckboxGroupState::default(),
            state_script: CheckboxGroupState::default(),
            active_area: ActiveArea::Project,
            message_log: MessageLog::new(),
            spin: Spin::new(SpinState::new(0, false)),
            exit: false,
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
            let _ = self.event_bus.0.send(AppEvent::Log("failed to load setting.json".into(), MessageType::Error));
            return;
        }

        let tx: mpsc::Sender<AppEvent> = self.event_bus.0.clone();
        std::thread::spawn(
            move ||
            {
                if let Ok(updater) = Updater::new().fetch()
                {
                    let _ = tx.send(AppEvent::UpdateAvailable(
                        ReleaseUpdateGithub
                        { 
                            version_current: updater.state.version_current, version_new: updater.state.version_new, is_available: updater.state.is_available
                        }
                    ));
                }
            }
        );

        for projects in json["projects"].as_array().unwrap_or(&vec![])
        {
            let mut configures: Vec<Configure> = Vec::new();
            for configure in projects["configures"].as_array().unwrap_or(&vec![])
            {
                let extension_mask: Vec<String> = configure["extension_mask"].as_array().unwrap_or(&vec![]).iter().filter_map(|v| v.as_str()).map(|s| s.replace("*.", "")) .collect();

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
                    extension_mask,
                    components,
                    scripts
                ));
            }

            self.projects.push(Project::new(
                projects["name"].as_str().unwrap_or_default().to_string(),
                projects["destination_path"].as_str().unwrap_or_default().to_string(),
                configures, 
                projects["selected"].as_bool().unwrap_or(false),
            ));
        }

        let _ = self.event_bus.0.send(AppEvent::Log("initialized 'mvtool'".into(), MessageType::Success));
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>
    {
        self.init();

        while !self.exit
        {
            terminal.draw(|f: &mut Frame<'_>| self.draw(f))?;
            
            self.handle_updates();
            self.spin.tick();

            if event::poll(std::time::Duration::from_millis(16))?
            {
                self.handle_events()?;
            }
        }

        return Ok(());
    }

    fn run_update(tx: mpsc::Sender<AppEvent>)
    {   
        let _ = tx.send(AppEvent::WaitProcess(SpinState { tick_count: 0, procces: true }));
        let _ = tx.send(AppEvent::Log("starting update process...".into(), MessageType::Info));

        std::thread::spawn(
            move ||
            {
                let result: Result<(), String> = (|| -> Result<(), String>
                {
                    let updater: Updater = Updater::new().fetch().map_err(|error| format!("fetch was cancelled for the following reasons: {error}"))?;
                    updater.update(tx.clone()).map_err(|error| format!("update process was aborted with the following error: {error}"))?;

                    return Ok(());
                }
            )();

            match result
            {
                Ok(_) => std::process::exit(0),
                Err(error) =>
                {
                    let _ = tx.send(AppEvent::Log(error, MessageType::Error));
                    let _ = tx.send(AppEvent::WaitProcess(SpinState { tick_count: 0, procces: false }));
                }
            }
        });
    }

    fn run_copying(projects: Vec<Project>, tx: mpsc::Sender<AppEvent>)
    {
        let _ = tx.send(AppEvent::Log("starting...".into(), MessageType::Info));
        let _ = tx.send(AppEvent::WaitProcess(SpinState { tick_count: 0, procces: true }));

        std::thread::spawn(
            move ||
            {
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
                            let _ = tx.send(AppEvent::Log("cleaning components...".into(), MessageType::Info));

                            if let Ok(entries) = fs::read_dir(dest_path)
                            {
                                for entry in entries.flatten()
                                {
                                    let file_name: String = entry.file_name().to_string_lossy().to_string();
                                    
                                    let matches_mask: bool = configure.get_extension_mask().is_empty() || configure.get_extension_mask().iter().any(|ext: &String| file_name.ends_with(ext));
                                    if !matches_mask
                                    {
                                        continue;
                                    }

                                    for component in configure.get_components()
                                    {
                                        if Utils::is_match(&entry.path(), component.get_name(), configure.get_extension_mask())
                                        {
                                            let _ = fs::remove_file(entry.path());
                                            break; 
                                        }
                                    }
                                }
                            }
                        }

                        let mut files_to_copy: Vec<(std::path::PathBuf, String)> = Vec::new();
                        if let Ok(entries) = fs::read_dir(src_path)
                        {
                            for entry in entries.flatten()
                            {
                                let file_name: String = entry.file_name().to_string_lossy().to_string();

                                let matches_mask: bool = configure.get_extension_mask().is_empty() || configure.get_extension_mask().iter().any(|ext: &String| file_name.ends_with(ext));
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

                                    if Utils::is_match(&entry.path(), component.get_name(), configure.get_extension_mask())
                                    {
                                        files_to_copy.push((entry.path(), file_name.clone()));
                                        break;
                                    }
                                }
                            }
                        }
                        else
                        {
                            let _ = tx.send(AppEvent::Log(format!("failed to read source directory: '{}'", src_path), MessageType::Warning));
                        }

                        let copied_total: usize = files_to_copy.len();
                        let mut copied_count: usize = 0;
                        if copied_total == 0
                        {
                            let _ = tx.send(AppEvent::Log("no files matched the criteria".into(), MessageType::Info));
                        }
                        else
                        {
                            for (path, file_name) in files_to_copy
                            {
                                let to: std::path::PathBuf = std::path::Path::new(dest_path).join(&file_name);

                                if fs::copy(&path, to).is_ok()
                                {
                                    copied_count = copied_count.add(1);
                                    let _ = tx.send(AppEvent::Log(format!("[{}/{}] copying: {}", copied_count, copied_total, file_name), MessageType::Info));
                                }
                            }

                            let _ = tx.send(AppEvent::Log(format!("finished copying {}/{}: '{}'", copied_count, copied_total, dest_path), MessageType::Success));
                        }
                    }
                }

                if fallback
                {
                    let _ = tx.send(AppEvent::Log("no project or configure selected, nothing was done".into(), MessageType::Warning));
                }

                let _ = tx.send(AppEvent::WaitProcess(SpinState { tick_count: 0, procces: false }));
            }
        );
    }

    fn handle_updates(&mut self)
    {
        while let Ok(event) = self.event_bus.1.try_recv()
        {
            match event
            {
                AppEvent::UpdateAvailable(state) =>
                {
                    if state.is_available
                    {
                        let tx: mpsc::Sender<AppEvent> = self.event_bus.0.clone();
                        self.message_box = Some(MessageBox::new("[ update ]", 
                            vec![
                                Line::from("New version available!".bold()).alignment(Alignment::Center),
                                Line::from(""),
                                Line::from(format!("current version: v.{}", state.version_current)).alignment(Alignment::Center),
                                Line::from(format!("new version: v.{}", state.version_new)).alignment(Alignment::Center),
                            ].into()
                            )
                            .with_size(50, 25)
                            .set_accept(
                                {
                                    move || { Self::run_update(tx.clone()); }
                                } 
                            )
                        );
                    }
                }
                AppEvent::WaitProcess(state) =>
                {
                    self.spin.state = state;
                }
                AppEvent::Log(message, message_type) =>
                {
                    self.message_log.add_message(message, message_type);
                }
            }
        }
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
            if key.kind != KeyEventKind::Press
            {
                return Ok(());
            }

            if let Some(message_box) = self.message_box.take()
            {
                match key.code
                {
                    KeyCode::Enter => message_box.accept(),
                    KeyCode::Esc   => message_box.reject(),

                    _ => { self.message_box = Some(message_box); }
                }

                return Ok(());
            }
        
            match key.code
            {
                KeyCode::Up         => self.move_selection(-1),
                KeyCode::Down       => self.move_selection(1),
                KeyCode::Left       => self.move_project(-1),
                KeyCode::Right      => self.move_project(1),
                KeyCode::Char(' ')  => self.on_action(),
                KeyCode::Tab        => self.next_area(true),
                KeyCode::BackTab    => self.next_area(false),
                KeyCode::F(1)       if !self.spin.state.procces => self.ok(),
                KeyCode::Esc        if !self.spin.state.procces => self.exit = true,

                _ => {}
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
                let len: usize = self.projects[self.state_project.cursor].get_configures().len();
                if len == 0
                {
                    return;
                }

                self.state_configure.cursor = (self.state_configure.cursor as i32 + delta).clamp(0, len as i32 - 1) as usize;
            }
            ActiveArea::Component =>
            {
                let len: usize = self.projects[self.state_project.cursor].get_configures()[self.state_configure.cursor].get_components().len();
                if len == 0
                {
                    return;
                }

                self.state_component.cursor = (self.state_component.cursor as i32 + delta).clamp(0, len as i32 - 1) as usize;
            }
            ActiveArea::Scripts =>
            {
                let len: usize = self.projects[self.state_project.cursor].get_configures()[self.state_configure.cursor].get_scripts().len();
                if len == 0
                {
                    return;
                }

                self.state_script.cursor = (self.state_script.cursor as i32 + delta).clamp(0, len as i32 - 1) as usize;
            }

            _ => {}
        }
    }

    fn move_project(&mut self, delta: i32)
    {
        if self.active_area == ActiveArea::Project && !self.projects.is_empty()
        {
            self.state_project.cursor = (self.state_project.cursor as i32 + delta).clamp(0, self.projects.len() as i32 - 1) as usize;
            self.state_configure.cursor = 0;
        }
    }

    fn next_area(&mut self, forward: bool)
    {
        match self.active_area
        {
            ActiveArea::Project =>
            {
                self.state_project.cursor = self.state_project.selected;
            }
            ActiveArea::Configure =>
            {
                self.state_configure.cursor = self.state_configure.selected;
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

                self.state_project.selected = self.state_project.cursor;
                let selected: bool = self.projects[self.state_project.cursor].is_selected();
                self.projects.iter_mut().for_each(
                    |project: &mut Project|
                    {
                        project.set_selected(false);
                    }
                );
                self.projects[self.state_project.cursor].set_selected(!selected);
            }
            ActiveArea::Configure =>
            {
                if self.projects[self.state_project.cursor].get_configures().is_empty()
                {
                    return;
                }
                
                let project: &mut Project = &mut self.projects[self.state_project.cursor];
                
                self.state_configure.selected = self.state_configure.cursor;
                let selected: bool = project.get_configures()[self.state_configure.cursor].is_selected();
                project.get_configures_mut().iter_mut().for_each(|configure: &mut Configure| { configure.set_selected(false); });
                project.get_configures_mut()[self.state_configure.cursor].set_selected(!selected);
            }
            ActiveArea::Component =>
            {
                if self.projects[self.state_project.cursor].get_configures()[self.state_configure.cursor].get_components().is_empty()
                {
                    return;
                }

                let component: &mut objects::Component = &mut self.projects[self.state_project.cursor].get_configures_mut()[self.state_configure.cursor].get_components_mut()[self.state_component.cursor];
                component.set_selected(!component.is_selected());
            }
            ActiveArea::Scripts =>
            {
                if self.projects[self.state_project.cursor].get_configures()[self.state_configure.cursor].get_scripts().is_empty()
                {
                    return;
                }

                let script: &Script = &self.projects[self.state_project.cursor].get_configures()[self.state_configure.cursor].get_scripts()[self.state_script.cursor];
                let config: &Configure = &self.projects[self.state_project.cursor].get_configures()[self.state_configure.cursor];

                let _ = self.event_bus.0.send(AppEvent::Log(format!("running: {}", script.get_name()), MessageType::Info));

                let status: Result<std::process::ExitStatus, io::Error> = Command::new("cmd").args(
                    ["/C", "start", format!("mvtool: {}", script.get_name()).as_str(), "cmd", "/K", script.get_command()]).current_dir(config.get_path()
                ).status();

                match status
                {
                    Ok(stat)
                    if stat.success() =>
                    {
                        let _ = self.event_bus.0.send(AppEvent::Log(format!("executed script: {}", script.get_name()), MessageType::Success));
                    }
                    _ =>
                    {
                        let _ = self.event_bus.0.send(AppEvent::Log(format!("failed to run or script error: {}", script.get_name()), MessageType::Error));
                    }
                }
            }
        }
    }

    fn ok(&mut self)
    {
        Self::run_copying(self.projects.clone(), self.event_bus.0.clone());
    }
}

impl Widget for &App
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let [project_area, middle_area, console_area, bottom_bar_area] = Layout::vertical([
            Constraint::Length(6),
            Constraint::Min(1),
            Constraint::Length(5),
            Constraint::Length(1),
        ]).areas(area);

        let [side_area, component_area] = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).areas(middle_area);
        let [configure_area, script_area] = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(side_area);

        // PROJECT LIST
        let mut project_group: HorizontalCheckboxGroup<'_> = HorizontalCheckboxGroup::new();
        let project_block: Block<'_> = Block::bordered().title("[ projects ] - use ← → to navigate ".bold()).border_set(border::ROUNDED).border_style(self.area_style(ActiveArea::Project)).padding(Padding { left: 2, right: 2, top: 1, bottom: 0 });

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
        let console_block: Block<'_> = Block::bordered().title(format!("[ console {}]", &mut self.spin.get_frame())).border_set(border::ROUNDED).padding(Padding { left: 1, right: 0, top: 1, bottom: 1 });

        // BOTTOM BAR
        let bottom_bar_help_menu: Paragraph<'_> = Paragraph::new(
            Line::from(vec!
                [
                    " F1 ".black().on_gray(), " Run ".gray(),
                    " Space ".black().on_gray(), " Toggle ".gray(),
                    " Tab ".black().on_gray(), " Next Area ".gray(),
                    " ▲ ▼ ".black().on_gray(), " Move ".gray(),
                    " Esc ".black().on_gray(), " Exit ".gray(),
                ]
        )).style(Style::default().bg(Color::Reset)).alignment(ratatui::layout::Alignment::Left);
        let bottom_bar_version: Paragraph<'_> = Paragraph::new(" [esud] mvtool v1.3.0 ".gray()).alignment(ratatui::layout::Alignment::Right);

        if !self.projects.is_empty()
        {
            // PROJECT LIST
            for (i, project) in self.projects.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(project.is_selected());
                if self.active_area == ActiveArea::Project
                {
                    state.focus();
                    if i == self.state_project.cursor
                    {
                        state.highlight();
                    }
                }  

                project_group.add_checkbox(
                    {
                        let mut checkbox: Checkbox<'_> = Checkbox::new(project.get_name());
                        checkbox.set_state(state);
                        checkbox
                    }
                );
            }

            // CONFIGURE LIST
            let configures: &Vec<Configure> = self.projects[self.state_project.cursor].get_configures();
            for (i, configure) in configures.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(configure.is_selected());
                if self.active_area == ActiveArea::Configure
                {
                    state.focus();
                    if i == self.state_configure.cursor
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
            let components: &Vec<objects::Component> = configures[self.state_configure.cursor].get_components();
            for (i, component) in components.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(component.is_selected());
                if self.active_area == ActiveArea::Component
                {
                    state.focus();
                    if i == self.state_component.cursor
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
            let scripts: &Vec<Script> = configures[self.state_configure.cursor].get_scripts();
            for (i, script) in scripts.iter().enumerate()
            {
                let mut state: CheckboxState = CheckboxState::new(false);
                state.data.symbols = Some(("", "→"));
                state.data.style_highlighted = Some(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

                if self.active_area == ActiveArea::Scripts
                {
                    state.focus();
                    if i == self.state_script.cursor
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
        
        // RENDER SUBLAYOUT 0
        let mut state_project: CheckboxGroupState = self.state_project.clone();
        project_group.render(project_block.inner(project_area), buf, &mut state_project);
        project_block.render(project_area, buf);

        // RENDER SUBLAYOUT 1
        let mut state_component: CheckboxGroupState = self.state_component.clone();
        component_group.render(component_block.inner(component_area), buf, &mut state_component);
        component_block.render(component_area, buf);

        // RENDER SUBLAYOUT 2
        let mut state_configure: CheckboxGroupState = self.state_configure.clone();
        configure_group.render(configure_block.inner(configure_area), buf, &mut state_configure);
        configure_block.render(configure_area, buf);

        // RENDER SUBLAYOUT 3
        let mut state_script: CheckboxGroupState = self.state_script.clone();
        script_group.render(script_block.inner(script_area), buf, &mut state_script);
        script_block.render(script_area, buf);

        // RENDER MAINLAYOUT 0
        console.render(console_area, buf);

        // RENDER MAINLAYOUT 1
        bottom_bar_help_menu.render(bottom_bar_area, buf);
        bottom_bar_version.render(bottom_bar_area, buf);

        // RENDER MESSAGEBOX
        if let Some(message_box) = self.message_box.as_ref()
        {
            Widget::render(message_box, area, buf);
        }
    }
}
