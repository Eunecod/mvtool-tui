// source/application/task/mod.rs

use std::fs;
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use std::path::PathBuf;

use crate::models::Project;

use crate::application::AppEvent;

use crate::widgets::console::MessageType;
use crate::widgets::helper::waiter::WaiterState;
use crate::widgets::helper::toast::ToastPayload;

use crate::service::is_match;

pub fn run_copying(projects: Vec<Project>, tx: mpsc::Sender<AppEvent>) {
    let _ = tx.send(AppEvent::Devent("Начало копирования...".into(), MessageType::Info));
    let _ = tx.send(AppEvent::WaitProcess(WaiterState { tick_count: 0, process: true }));

    thread::spawn(move || {
        let mut fallback = true;

        for project in projects.iter().filter(|project| project.selected) {
            let dest_path = &project.destination_path;

            for configure in project.configures.iter().filter(|configure| configure.selected) {
                fallback = false;
                let src_path = &configure.source_path;

                if configure.clean_destination {
                    let _ = tx.send(AppEvent::Devent("Очистка целевой директории...".into(), MessageType::Info));

                    if let Ok(entries) = fs::read_dir(dest_path) {
                        for entry in entries.flatten() {
                            let path = entry.path();

                            for component in &configure.components {
                                if is_match(&path, &component.name, &configure.extension_mask) {
                                    let _ = fs::remove_file(&path);
                                    break;
                                }
                            }
                        }
                    }
                }

                let mut files_to_copy: Vec<(PathBuf, String)> = Vec::new();
                let entries = match fs::read_dir(src_path) {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        let _ = tx.send(AppEvent::Devent(format!("Не удалось прочитать '{}': {}", src_path, error), MessageType::Warning));
                        continue;
                    }
                };

                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = entry.file_name();

                    let file_name_str = file_name.to_string_lossy();

                    let matches_mask = configure.extension_mask.is_empty()
                        || configure.extension_mask.iter().any(|mask| {
                            let mask = mask.trim_start_matches("*.");
                            file_name_str.ends_with(mask)
                        });

                    if !matches_mask {
                        continue;
                    }

                    for component in configure.components.iter().filter(|c| c.selected) {
                        if is_match(&path, &component.name, &configure.extension_mask) {
                            files_to_copy.push((path, file_name_str.to_string()));
                            break;
                        }
                    }
                }

                let total = files_to_copy.len();
                let mut done = 0;

                if total == 0 {
                    let _ = tx.send(AppEvent::Devent("Файлы для копирования не найдены".into(), MessageType::Info));
                    continue;
                }

                for (path, file_name) in files_to_copy {
                    let to = Path::new(dest_path).join(&file_name);

                    if fs::copy(&path, &to).is_ok() {
                        done += 1;
                        let _ = tx.send(AppEvent::Devent(format!("[{}/{}] {}", done, total, file_name), MessageType::Info));
                    }
                }

                let _ = tx.send(AppEvent::Devent(format!("Готово {}/{} в '{}'", done, total, dest_path), MessageType::Success));
            }
        }

        if fallback {
            let _ = tx.send(AppEvent::Devent("Ничего не выбрано".into(), MessageType::Warning));
        }

        let _ = tx.send(AppEvent::WaitProcess(WaiterState { tick_count: 0, process: false }));
        let _ = tx.send(AppEvent::ShowToast(ToastPayload {
            title: "Копирование завершено".into(),
            message: "Все файлы успешно скопированы".into(),
            duration: 3.0,
        }));
    });
}