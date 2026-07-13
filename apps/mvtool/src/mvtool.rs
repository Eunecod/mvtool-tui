// mvtool/src/mvtool.rs

use tokio::process::Command as AsyncCommand;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::Key;
use winit::keyboard::NamedKey;
use winit::window::Icon;
use winit::window::Window;
use winit::window::WindowId;

use mvcore::events::Command;
use mvcore::events::Type;

use mvcore::session::UpdateSession;

use mvcore::io::JsonRepository;
use mvcore::io::Repository;
use mvcore::io::Root;

use mvframe::backend::RenderContext;
use mvframe::backend::UiState;

use mvframe::widget::AboutWidget;
use mvframe::widget::ComponentsWidget;
use mvframe::widget::ConfiguresWidget;
use mvframe::widget::ConsoleWidget;
use mvframe::widget::MessageBox;
use mvframe::widget::PluginManagerWidget;
use mvframe::widget::PluginWidget;
use mvframe::widget::ProjectsWidget;
use mvframe::widget::ScriptsWidget;
use mvframe::widget::UpdatesWidget;
use mvframe::widget::Widget;

use mvframe::widget::plugins::ItemMenuData;
use mvframe::widget::plugins::ItemMenuPlugins;

use mvplugin::api::Plugin;
use mvplugin::system::PluginManager;

use updater::Updater;

use crate::domain::SharedData;

pub struct Application {
    rt: Runtime,
    rx: mpsc::Receiver<Command>,
    tx: mpsc::Sender<Command>,

    plugin_manager: PluginManager,

    ui: UiState,
    gfx: Option<RenderContext>,
    window: Option<Window>,
    update_session: Option<UpdateSession>,
    data: SharedData,

    about: AboutWidget,
    console: ConsoleWidget,
    projects: ProjectsWidget,
    configures: ConfiguresWidget,
    components: ComponentsWidget,
    scripts: ScriptsWidget,
    updates: UpdatesWidget,
    plugins: Vec<PluginWidget>,
    item_menu_plugins: Vec<ItemMenuData>,
    plugin_manager_widget: PluginManagerWidget,

    messagebox: MessageBox,
}

impl Application {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        let rt = Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .expect("Failed to create runtime");

        let tx_plugins = tx.clone();
        let tx_scripts = tx.clone();
        let tx_updates = tx.clone();

        Self {
            rt,
            rx,
            tx,
            plugin_manager: PluginManager::new(tx_plugins),
            ui: UiState::new(),
            gfx: None,
            window: None,
            update_session: None,
            data: SharedData::new(Root {
                projects: Vec::new(),
            }),
            about: AboutWidget::new(),
            console: ConsoleWidget::new(),
            projects: ProjectsWidget::new(),
            configures: ConfiguresWidget::new(),
            components: ComponentsWidget::new(),
            scripts: ScriptsWidget::new(tx_scripts),
            updates: UpdatesWidget::new(tx_updates),
            plugins: Vec::new(),
            item_menu_plugins: Vec::new(),
            plugin_manager_widget: PluginManagerWidget::new(),
            messagebox: MessageBox::new(),
        }
    }

    pub fn init(mut self) -> Self {
        self.ui.setup();
        self.ui
            .setup_fonts(include_bytes!("platforms/font/mvtool-bold.ttf"));

        self.try_update();

        #[cfg(windows)]
        {
            use mvcore::service::register_aumid;
            match register_aumid("mvtool") {
                Ok(_) => (),
                Err(error) => {
                    let _ = self.tx.blocking_send(Command::Devent(
                        format!("Не удалось зарегистрировать aumid приложения: {}", error),
                        Type::Warning,
                    ));
                }
            }
        }

        let repository = JsonRepository::new("setting.json");
        match repository.load() {
            Ok(root) => {
                let mut data = self.data.write();
                *data = root;
            }
            Err(error) => {
                let _ = self.tx.blocking_send(Command::Devent(error, Type::Error));
                return self;
            }
        }

        if let Err(error) = self.plugin_manager.loader.load_plugins("./plugins") {
            let _ = self.tx.blocking_send(Command::Devent(error, Type::Error));
        } else if !self.plugin_manager.loader.plugins.is_empty() {
            let _ = self.tx.blocking_send(Command::Devent(
                format!(
                    "Плагины успешно загруженны [загруженно {}]",
                    self.plugin_manager.loader.plugins.len()
                ),
                Type::Success,
            ));

            for (name, plugin) in &self.plugin_manager.loader.plugins {
                let table = plugin.hook();
                if let Ok(_) = table.get::<mlua::Function>("build") {
                    self.plugins.push(PluginWidget::new(&name));
                }
                if let Ok(_) = table.get::<mlua::Function>("menu") {
                    self.item_menu_plugins.push(ItemMenuData::new(&name));
                }
            }
        }

        self.console.set_auto_scroll(true);

        let _ = self.tx.blocking_send(Command::Devent(
            "Программа 'mvtool' готова к работе".into(),
            Type::Success,
        ));
        self
    }

    pub fn bootstrap() -> Self {
        Self::new().init()
    }

    pub fn run(&mut self) {
        let event_loop = match EventLoop::new() {
            Ok(instance) => instance,
            Err(error) => {
                let _ = self.tx.blocking_send(Command::Devent(
                    format!("Не удалось создать главный цикл приложения: {}", error),
                    Type::Error,
                ));
                return;
            }
        };

        event_loop.set_control_flow(ControlFlow::Wait);
        let _ = event_loop.run_app(self);
    }

    fn load_icon(&mut self, raw_data: &[u8]) -> Option<Icon> {
        let icon = mvcore::service::load_icon(raw_data);
        Icon::from_rgba(icon.as_raw().to_vec(), icon.width(), icon.height()).ok()
    }

    fn event_process(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Command::Devent(message, message_type) => {
                    self.console.add(message.as_str(), message_type);
                    println!("{}", message);
                }
                Command::ShowNotification(title, message) => {
                    mvcore::service::show_notification("mvtool", &title, &message);
                }
                Command::ShowMessageBox(title, message, action) => {
                    self.messagebox = MessageBox::new()
                        .title(title)
                        .message(message)
                        .spawn(action);
                }
                Command::Execute(name, command) => {
                    let tx = self.tx.clone();

                    self.rt.spawn(async move {
                        let process_execute = {
                            #[cfg(target_os = "windows")]
                            {
                                AsyncCommand::new("cmd")
                                    .args(&["/C", &command])
                                    .current_dir(".")
                                    .spawn()
                            }
                            #[cfg(target_os = "linux")]
                            {
                                AsyncCommand::new("sh")
                                    .args(&["-c", &command])
                                    .current_dir(".")
                                    .spawn()
                            }
                        };

                        match process_execute {
                            Ok(mut process) => match process.wait().await {
                                Ok(status) if status.success() => {
                                    let _ = tx
                                        .send(Command::Devent(
                                            format!("Задача '{}' выполнена", name),
                                            Type::Success,
                                        ))
                                        .await;
                                }
                                _ => {
                                    let _ = tx
                                        .send(Command::Devent(
                                            format!("Задача '{}' завершилась с ошибкой", name),
                                            Type::Error,
                                        ))
                                        .await;
                                }
                            },
                            Err(error) => {
                                let _ = tx
                                    .send(Command::Devent(
                                        format!("Ошибка при запуске задачи '{}': {}", name, error),
                                        Type::Error,
                                    ))
                                    .await;
                            }
                        }
                    });
                }
                Command::Copyng() => {
                    let _ = self.tx.blocking_send(Command::Devent(
                        "Начало копирования...".into(),
                        Type::Message,
                    ));

                    let tx = self.tx.clone();
                    let projects = self.data.read().projects.clone();

                    self.rt.spawn(async move {
                        mvcore::task::copying(projects, tx).await;

                        mvcore::service::show_notification(
                            "mvtool",
                            "Копирование завершено",
                            "Все файлы успешно скопированы",
                        );
                    });
                }
                Command::UpdaterReady(update_session) => {
                    if let Some(updater) = update_session.as_ref_concrete::<Updater>() {
                        self.updates
                            .set_version(updater.get_latest_version().into());
                        self.updates.set_link(updater.get_release_link().into());
                    }

                    self.update_session = Some(update_session);
                    self.updates.open();
                }
                Command::Update() => {
                    let _ = self
                        .tx
                        .blocking_send(Command::Devent("Обновление...".into(), Type::Message));

                    let tx = self.tx.clone();
                    if let Some(session) = self.update_session.take() {
                        self.rt.spawn(async move {
                            if let Err(error) = session
                                .extract::<Updater>()
                                .expect("REASON")
                                .update(tx.clone())
                                .await
                                .map_err(|error| format!("Ошибка обновления: {}", error))
                            {
                                let _ = tx.send(Command::Devent(error, Type::Error)).await;
                            }

                            let mvtool_exe =
                                match std::env::current_exe() {
                                    Ok(file_exe) => file_exe,
                                    Err(error) => {
                                        let _ = tx.send(Command::Devent(
                                        format!(
                                            "Не удалось получить путь к исполняемому файлу: {}",
                                            error.to_string()
                                        ),
                                        Type::Error,
                                    )).await;

                                        return;
                                    }
                                };

                            if let Err(error) = AsyncCommand::new(mvtool_exe).spawn() {
                                let _ = tx
                                    .send(Command::Devent(
                                        format!("Не удалось перезапустить программу: {}", error),
                                        Type::Error,
                                    ))
                                    .await;
                            }

                            let _ = tx.send(Command::Exit()).await;
                        });
                    }
                }
                Command::PluginManager() => {
                    self.plugin_manager_widget.open();
                }
                Command::Exit() => {
                    event_loop.exit();
                }
                Command::About() => {
                    self.about.open();
                }
            }
        }
    }

    fn try_update(&mut self) {
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            match Updater::new().await {
                Ok(updater) => {
                    if updater.is_update_available() {
                        let session = UpdateSession::new(updater);
                        let _ = tx.send(Command::UpdaterReady(session)).await;
                    }
                }
                Err(error) => {
                    let _ = tx
                        .send(Command::Devent(
                            format!(
                                "Ошибка подключения к GitHub при проверке обновлений: {}",
                                error
                            ),
                            Type::Warning,
                        ))
                        .await;
                }
            }
        });
    }

    fn render(&mut self) {
        if let Some(window) = &mut self.window {
            self.ui.build(window, |ui| {
                ui.dockspace_over_main_viewport();

                self.console.draw(ui, &mut ());
                {
                    let mut data = self.data.write();
                    self.projects.draw(ui, &mut data.projects);

                    ui.main_menu_bar(|| {
                        ui.menu("Файл", || {
                            if ui.menu_item("Менеджер плагинов") {
                                let _ = self.tx.blocking_send(Command::PluginManager());
                            }
                            if ui.menu_item("Настройки") {}

                            ui.separator();

                            if ui.menu_item("Выход") {
                                let _ = self.tx.blocking_send(Command::Exit());
                            }
                        });

                        if !self.item_menu_plugins.is_empty() {
                            ui.menu("Плагины", || {
                                for item_menu in &mut self.item_menu_plugins {
                                    if let Some(_) = ui.begin_menu(&item_menu.title) {
                                        let mut widget = ItemMenuPlugins {
                                            data: item_menu,
                                            plugin_manager: &mut self.plugin_manager,
                                        };
                                        widget.draw(ui, &mut data.projects);
                                    }
                                }
                            });
                        }

                        ui.menu("О программе", || {
                            if ui.menu_item("Что это?!") {
                                let _ = self.tx.blocking_send(Command::About());
                            }
                        });
                    });

                    for plugin in &mut self.plugins {
                        plugin.draw(ui, &self.plugin_manager, &mut data.projects);
                    }

                    if let Some(project) = data.projects.iter_mut().find(|project| project.selected)
                    {
                        self.configures.draw(ui, &mut project.configures);
                        if let Some(configure) = project
                            .configures
                            .iter_mut()
                            .find(|configure| configure.selected)
                        {
                            self.components.draw(ui, &mut configure.components);
                            self.scripts.draw(ui, &mut configure.scripts);
                        }
                    }
                }

                self.updates.draw(ui, &mut ());
                self.about.draw(ui, &mut ());

                let mut plugins: Vec<&mut Plugin> =
                    self.plugin_manager.loader.plugins.values_mut().collect();

                self.plugin_manager_widget.draw(ui, &mut plugins);

                self.messagebox.draw(ui, &mut ());
            });
        }

        if let Some(paint) = &mut self.gfx {
            paint.render(&self.ui.data());
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("mvtool")
            .with_inner_size(PhysicalSize::new(1280.0, 1024.0))
            .with_window_icon(self.load_icon(include_bytes!("platforms/windows/icon.ico")))
            .with_visible(false);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        self.ui.attach_window(&window);

        let paint = self
            .rt
            .block_on(async { RenderContext::new(&mut self.ui.context(), &window).await });

        window.set_visible(true);

        self.window = Some(window);
        self.gfx = Some(paint);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.window {
            Some(window) if window.id() == window_id => window,
            _ => return,
        };
        self.ui.handle_event(window, &event);

        match event {
            WindowEvent::Resized(_) => {
                if let Some(paint) = &mut self.gfx {
                    paint.resize(window.inner_size().width, window.inner_size().height);
                }
            }
            WindowEvent::CloseRequested => {
                let _ = self
                    .tx
                    .blocking_send(Command::Devent("Закрытие приложения".into(), Type::Success));
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::F1),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                let _ = self.tx.blocking_send(Command::Copyng());
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::F2),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                let data = self.data.read().projects.clone();

                if let Err(error) = self.plugin_manager.emit("on_start", &data, None) {
                    let _ = self.tx.blocking_send(Command::Devent(error, Type::Error));
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                let _ = self.tx.blocking_send(Command::Exit());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key {
                    let imgui_key = self.ui.key(key_code);

                    if let Some(key) = imgui_key {
                        self.ui
                            .context()
                            .io_mut()
                            .add_key_event(key, event.state.is_pressed());
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.event_process(event_loop);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
