// source/application/mod.rs

use std::process::Command;
use std::process::exit;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::env::current_exe;

use glow::HasContext;

mod backend;
use backend::_imgui::ImGUI;
use backend::_sdl2::SDL2;
use backend::_opengl::OpenGL;

pub mod events;
use events::AppEvent;

mod repository;
use repository::Repository;
use repository::JsonRepository;

mod task;
use task::run_copying;

use crate::models::Root;

#[cfg(windows)]
use crate::service::register_aumid;

use crate::system::plugin::PluginLoader;
use crate::system::plugin::PluginManager;
use crate::system::plugin::Api;

use crate::system::updater::Updater;

use crate::widgets::Widget;
use crate::widgets::console::ConsoleWidget;
use crate::widgets::console::MessageType;
use crate::widgets::projects::ProjectsWidget;
use crate::widgets::configures::ConfiguresWidget;
use crate::widgets::components::ComponentsWidget;
use crate::widgets::scripts::ScriptsWidget;
use crate::widgets::about::AboutWidget;
use crate::widgets::messagebox::SimpleMessageBox;

use crate::widgets::helper::waiter::WaiterState;
use crate::widgets::helper::toast::ToastWidget;

pub struct Workspace {
	pub console_widget: ConsoleWidget,
	pub projects_widget: ProjectsWidget,
	pub configures_widget: ConfiguresWidget,
	pub components_widget: ComponentsWidget,
	pub scripts_widget: ScriptsWidget,

	pub about_widget: AboutWidget,
}

impl Workspace {
	pub fn new() -> Self {
		Self {
			console_widget: ConsoleWidget::new(),
			projects_widget: ProjectsWidget::new(),
			configures_widget: ConfiguresWidget::new(),
			components_widget: ComponentsWidget::new(),
			scripts_widget: ScriptsWidget::new(),

			about_widget: AboutWidget::new(),
		}
	}
}

pub struct Systems {
	plugin_loader: PluginLoader,
	plugin_manager: PluginManager
}

impl Systems {
	pub fn new(api: Api) -> Result<Self, String> {
		Ok(Self { plugin_loader: PluginLoader::new(api)?, plugin_manager: PluginManager::new() })
	}
}

#[derive(Clone)]
pub struct SharedRoot {
    inner: Arc<RwLock<Root>>
}

impl SharedRoot {
    pub fn new(root: Root) -> Self {
        Self { inner: Arc::new(RwLock::new(root)) }
    }

    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Root> {
        self.inner.read().expect("Root lock poisoned")
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, Root> {
        self.inner.write().expect("Root lock poisoned")
    }
}

pub struct Application {
	event_bus: (mpsc::Sender<AppEvent>, mpsc::Receiver<AppEvent>),
	systems: Systems,

	waiter_state: WaiterState,
	message_box: SimpleMessageBox,

	imgui: ImGUI,
	sdl: SDL2,
	workspace: Workspace,
	data: SharedRoot
}

impl Application {
	pub fn new() -> Result<Self, String> {
		let (sender, receiver) = mpsc::channel();

		let shared_data = SharedRoot::new(Root { projects: Vec::new() });

		let api = Api {
			sender: sender.clone(),
			data: shared_data.clone()
		};

		Ok(Self {
			event_bus: (sender, receiver),
			systems: Systems::new(api)?,

			waiter_state: WaiterState { tick_count: 0, process: false },
			message_box: SimpleMessageBox { title: "".to_string(), message: "".to_string(), is_open: false, on_yes: None, on_no: None },

			imgui: ImGUI::new(),
			sdl: SDL2::new()?,
			workspace: Workspace::new(),
			data: shared_data,
		})
	}

	pub fn init(mut self) -> Self {
		self.imgui.setup_ui();
		self.try_update();

		#[cfg(windows)]
		{
			match register_aumid("com.mvtool.desktop", "mvtool") {
				Ok(_) => { }
				Err(error) => {
					let _ = self.event_bus.0.send(AppEvent::Devent(format!("Не удалось зарегистрировать aumid приложения: {}", error), MessageType::Warning));
				}
			}
		}

		if let Err(error) = self.systems.plugin_loader.load_plugins("./plugins") {
		    let _ = self.event_bus.0.send(AppEvent::Devent(error, MessageType::Error));
		}
		else if !self.systems.plugin_loader.plugins.is_empty() {
			let _ = self.event_bus.0.send(
				AppEvent::Devent(format!("Плагины успешно загруженны [загруженно {}]", self.systems.plugin_loader.plugins.len()), MessageType::Success)
			);
		}

		let repository = JsonRepository::new("setting.json");
		match repository.load() {
            Ok(root) => {
                let mut data = self.data.write();
                *data = root;
            }
            Err(error) => {
				let _ = self.event_bus.0.send(AppEvent::Devent(error, MessageType::Error));
				return self;
            }
        }

		self.workspace.console_widget.set_auto_scroll(true);
		
		let _ = self.event_bus.0.send(AppEvent::Devent("Программа 'mvtool' готова к работе".into(), MessageType::Success));
		self
	}

	pub fn run(&mut self) -> Result<(), String> {
		let mut running = true;

		let mut opengl = OpenGL::new(self.sdl.video(), self.imgui.context())?;
		let mut events_keyboard = Vec::new();

		'running: loop {
			events_keyboard.clear();
			self.sdl.poll_events(|event| {
				self.imgui.handle_event(&event);
			    if let sdl2::event::Event::Quit { .. } = event {
			        running = false;
			    }
			
				if let sdl2::event::Event::KeyDown { .. } = event {
				    events_keyboard.push(event);
				}
			});

			self.imgui.prepare_frame(&self.sdl.window(), &self.sdl.event_pump());
			self.keyboard(events_keyboard.clone())?;
			self.event_process();

			if !running {
				break 'running;
			}

			unsafe {
				let ctx = opengl.gl_renderer().gl_context();
			
				ctx.clear_color(0.1, 0.1, 0.1, 1.0);
				ctx.clear(glow::COLOR_BUFFER_BIT);
			}

			self.imgui.paint(|ui, root| {
				ui.dockspace_over_main_viewport();
				
				ui.main_menu_bar(|| {
				    ui.menu("Файл", || {
				        ui.menu_item("Менеджер плагинов");
				
						ui.separator();
				
						if ui.menu_item("Выход") {
				            let _ = self.sdl.event().push_event(sdl2::event::Event::Quit { timestamp: 0 });
				        }
				    });
				
				    ui.menu("О программе", || {
				        if ui.menu_item("Что это?!") {
							self.workspace.about_widget.is_open = true;
						}
					});
				
				});
				
				/* Рисуем виджеты рабочего пространства */
				{
					self.workspace.console_widget.pump_waiter(&mut self.waiter_state);
				
					self.workspace.console_widget.draw(ui, root);
					self.workspace.projects_widget.draw(ui, root);
					self.workspace.configures_widget.draw(ui, root);
					self.workspace.components_widget.draw(ui, root);
					self.workspace.scripts_widget.draw(ui, root);
				
					self.workspace.about_widget.draw(ui, root);
				}
			
				self.message_box.draw(ui, root);
			}, &mut self.data.write());

			opengl.gl_renderer().render(self.imgui.data())?;
			self.sdl.swap();
		}

		Ok(())
	}

	fn try_update(&mut self) {
		let sender = self.event_bus.0.clone();

		std::thread::spawn(move || {
		    match Updater::new() {
		        Ok(updater) => {
		            if updater.is_update_available() {
		                let tag_name = updater.get_latest_version().to_string();
		                let updater = Arc::new(updater);
		
		                let _ = sender.clone().send(AppEvent::ShowMessageBox(
		                    SimpleMessageBox {
		                        title: "Доступно обновление".into(),
		                        message: format!("Доступна новая версия {}. Хотите скачать?", tag_name),
		                        is_open: true,
		                        on_yes: Some(Box::new({
		                            move || {
										let sender = sender.clone();
										let updater = updater.clone();
		
		                                std::thread::spawn(move || {
		                                    if let Err(error) = updater.update(sender.clone()) {
		                                        let _ = sender.send(AppEvent::Devent(format!("Ошибка обновления: {}", error), MessageType::Error));
		                                    }
		
		                                    let exe = match current_exe() {
		                                        Ok(exe) => exe,
		                                        Err(error) => {
		                                            let _ = sender.send(AppEvent::Devent(format!("Не удалось получить путь к исполняемому файлу: {}", error), MessageType::Error));
		                                            return;
		                                        }
		                                    };
		
		                                    if let Err(error) = Command::new(exe).spawn() {
		                                        let _ = sender.send(AppEvent::Devent(format!("Не удалось перезапустить программу: {}", error), MessageType::Error));
		                                    }
		
		                                    exit(0);
		                                });
		                            }
		                        })),
		                        on_no: None,
		                    }
		                ));
		            }
		        }
		        Err(error) => {
		            let _ = sender.send(AppEvent::Devent(format!("Ошибка подключения к GitHub при проверке обновлений: {}", error), MessageType::Warning));
		        }
		    }
		});
	}

	fn event_process(&mut self) {
		while let Ok(event) = self.event_bus.1.try_recv() {
            match event {
                AppEvent::Devent(message, message_type) => {
                    self.workspace.console_widget.add(message.as_str(), message_type);
					println!("{}", message);
                }

				AppEvent::WaitProcess(state) => {
					self.waiter_state = state;
				}

				AppEvent::ShowToast(toast) => {
					ToastWidget::show(toast);
				}

				AppEvent::ShowMessageBox(message_box) => {
					self.message_box = message_box;
				}
			}
        }
	}

	fn keyboard(&mut self, events: Vec<sdl2::event::Event>) -> Result<(), String> {
		for event in events {
		    match event {
		        sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F1), repeat: false, .. } => {
		            if !self.waiter_state.process {
						self.systems.plugin_manager.emit(&self.systems.plugin_loader, "on_action", &mlua::Value::Nil)
							.map_err(|error| format!("Ошибка выполнения плагинами: {}", error))?;

		                run_copying(self.data.read().projects.clone(), self.event_bus.0.clone());
		            }
		        }

		        sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), repeat: false, .. } => {
		            self.sdl.event().push_event(sdl2::event::Event::Quit { timestamp: 0 })?;
		        }

		        _ => {}
		    }
		}

		Ok(())
	}
}
