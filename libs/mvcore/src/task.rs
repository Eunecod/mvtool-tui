// libs/core/src/task.rs

use crate::events::Command;
use crate::events::Type;
use crate::io::Project;

use tokio::fs;
use tokio::sync::mpsc;

use std::path::PathBuf;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use futures::stream;
use futures::stream::StreamExt;

const MAX_CONCURRENT_COPIES: usize = 8;

pub async fn copying(projects: Vec<Project>, tx: mpsc::Sender<Command>) {
    let mut fallback = true;

    for project in projects.iter().filter(|project| project.selected) {
        let dest_path = project.destination_path.clone();

        for configure in project
            .configures
            .iter()
            .filter(|configure| configure.selected)
        {
            fallback = false;

            let src_path = &configure.source_path;
            let src_path_buf = PathBuf::from(src_path);
            let tx = tx.clone();
            let dest_root_dir = PathBuf::from(&dest_path);

            if configure.clean_destination {
                let _ = tx
                    .send(Command::Devent(
                        "Очистка целевой директории...".into(),
                        Type::Message,
                    ))
                    .await;

                let dest_root_clone = dest_root_dir.clone();
                let components_clone = configure.components.clone();

                let _ = tokio::task::spawn_blocking(move || {
                    fn clean_recursive(dir: &std::path::Path, components: &[crate::io::Component]) {
                        if let Ok(entries) = std::fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_dir() {
                                    clean_recursive(&path, components);
                                } else if path.is_file() {
                                    if let Some(file_name) = path.file_name() {
                                        let file_name_str = file_name.to_string_lossy();
                                        for component in components {
                                            let clean_name = component.name.replace('/', "\\");
                                            let file_pattern = clean_name
                                                .split('\\')
                                                .last()
                                                .unwrap_or(&clean_name);

                                            if file_name_str.contains(file_pattern) {
                                                let _ = std::fs::remove_file(&path);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    clean_recursive(&dest_root_clone, &components_clone);
                })
                .await;
            }

            let mut files_to_copy: Vec<(PathBuf, String, Vec<String>)> = Vec::new();

            let selected_components: Vec<_> =
                configure.components.iter().filter(|c| c.selected).collect();

            if selected_components.is_empty() {
                let _ = tx
                    .send(Command::Devent(
                        "Не выбраны компоненты для копирования".into(),
                        Type::Warning,
                    ))
                    .await;
                continue;
            }

            for component in selected_components {
                let clean_name = component.name.replace('/', "\\");
                let parts: Vec<&str> = clean_name.split('\\').collect();

                let (dir_parts, file_pattern) = if parts.len() > 1 {
                    let (file, dirs) = parts.split_last().unwrap();
                    (dirs, *file)
                } else {
                    (&[][..], parts[0])
                };

                let mut target_dir = src_path_buf.clone();
                for p in dir_parts {
                    target_dir.push(p);
                }

                let sub_dirs: Vec<String> = dir_parts.iter().map(|s| s.to_string()).collect();

                if target_dir.is_dir() {
                    if let Ok(mut entries) = fs::read_dir(&target_dir).await {
                        while let Ok(Some(entry)) = entries.next_entry().await {
                            let path = entry.path();
                            if path.is_file() {
                                let file_name_str =
                                    entry.file_name().to_string_lossy().into_owned();

                                let matches_mask = configure.extension_mask.is_empty()
                                    || configure.extension_mask.iter().any(|mask| {
                                        let mask = mask.trim_start_matches("*");
                                        file_name_str.ends_with(mask)
                                    });

                                let matches_pattern = file_name_str.contains(file_pattern)
                                    || file_name_str.eq_ignore_ascii_case(file_pattern);

                                if matches_mask && matches_pattern {
                                    files_to_copy.push((path, file_name_str, sub_dirs.clone()));
                                }
                            }
                        }
                    }
                } else {
                    let target_file_path = src_path_buf.join(file_pattern);
                    if target_file_path.is_file() {
                        if let Some(file_name) = target_file_path.file_name() {
                            let file_name_str = file_name.to_string_lossy().into_owned();

                            let matches_mask = configure.extension_mask.is_empty()
                                || configure.extension_mask.iter().any(|mask| {
                                    let mask = mask.trim_start_matches("*");
                                    file_name_str.ends_with(mask)
                                });

                            if matches_mask {
                                files_to_copy.push((target_file_path, file_name_str, vec![]));
                            }
                        }
                    }
                }
            }

            let total = files_to_copy.len();
            if total == 0 {
                let _ = tx
                    .send(Command::Devent(
                        "Файлы для копирования не найдены".into(),
                        Type::Message,
                    ))
                    .await;
                continue;
            }

            let done_counter = Arc::new(AtomicUsize::new(0));
            let dest_path_clone = dest_path.clone();

            let copy_stream = stream::iter(files_to_copy).map(|(path, file_name, sub_dirs)| {
                let tx = tx.clone();
                let done_counter = Arc::clone(&done_counter);
                let dest_root_dir = PathBuf::from(&dest_path_clone);

                async move {
                    let mut target_dest_dir = dest_root_dir;
                    for sub in &sub_dirs {
                        target_dest_dir.push(sub);
                    }

                    if !target_dest_dir.is_dir() {
                        let _ = tx
                            .send(Command::Devent(
                                format!(
                                    "Целевая подпапка не существует: '{}'",
                                    target_dest_dir.display()
                                ),
                                Type::Warning,
                            ))
                            .await;
                        return;
                    }

                    let is_archive = file_name.ends_with(".tar.gz") || file_name.ends_with(".tar");

                    if is_archive {
                        match crate::service::unpackage_tar(&path, &target_dest_dir) {
                            Ok(_) => {
                                let current_done = done_counter.fetch_add(1, Ordering::SeqCst) + 1;
                                let _ = tx
                                    .send(Command::Devent(
                                        format!("[{}/{}] {}", current_done, total, file_name),
                                        Type::Message,
                                    ))
                                    .await;
                            }
                            Err(_) => {
                                let _ = tx
                                    .send(Command::Devent(
                                        format!("Не удалось распаковать архив '{}'", file_name),
                                        Type::Message,
                                    ))
                                    .await;
                            }
                        }
                    } else {
                        let to = target_dest_dir.join(&file_name);
                        match fs::copy(&path, &to).await {
                            Ok(_) => {
                                let current_done = done_counter.fetch_add(1, Ordering::SeqCst) + 1;
                                let _ = tx
                                    .send(Command::Devent(
                                        format!("[{}/{}] {}", current_done, total, file_name),
                                        Type::Message,
                                    ))
                                    .await;
                            }
                            Err(_) => {
                                let _ = tx
                                    .send(Command::Devent(
                                        format!("Не удалось скопировать '{}'", file_name),
                                        Type::Message,
                                    ))
                                    .await;
                            }
                        }
                    }
                }
            });

            copy_stream
                .buffer_unordered(MAX_CONCURRENT_COPIES)
                .collect::<()>()
                .await;

            let final_done = done_counter.load(Ordering::SeqCst);
            let _ = tx
                .send(Command::Devent(
                    format!("Готово {}/{} в '{}'", final_done, total, dest_path),
                    Type::Success,
                ))
                .await;
        }
    }

    if fallback {
        let _ = tx
            .send(Command::Devent("Ничего не выбрано".into(), Type::Warning))
            .await;
    }
}
