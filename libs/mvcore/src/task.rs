// libs/core/src/task.rs

use crate::events::Command;
use crate::events::Type;
use crate::io::Project;

use tokio::fs;
use tokio::sync::mpsc;

use std::path::Path;
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
            let tx = tx.clone();

            if configure.clean_destination {
                let _ = tx
                    .send(Command::Devent(
                        "Очистка целевой директории...".into(),
                        Type::Message,
                    ))
                    .await;

                if let Ok(mut entries) = fs::read_dir(&dest_path).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let path = entry.path();

                        for component in &configure.components {
                            if crate::service::is_match(
                                &path,
                                &component.name,
                                &configure.extension_mask,
                            ) {
                                if let Err(error) = fs::remove_file(&path).await {
                                    let _ = tx
                                        .send(Command::Devent(
                                            format!(
                                                "Проблема при удалении '{}': {}",
                                                component.name, error
                                            ),
                                            Type::Warning,
                                        ))
                                        .await;
                                }
                                break;
                            }
                        }
                    }
                }
            }

            let mut entries = match fs::read_dir(src_path).await {
                Ok(entry) => entry,
                Err(error) => {
                    let _ = tx
                        .send(Command::Devent(
                            format!("Не удалось прочитать '{}': {}", src_path, error),
                            Type::Warning,
                        ))
                        .await;
                    continue;
                }
            };

            let mut files_to_copy: Vec<(PathBuf, String)> = Vec::new();

            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                let file_name_str = entry.file_name().to_string_lossy().into_owned();

                let matches_mask = configure.extension_mask.is_empty()
                    || configure.extension_mask.iter().any(|mask| {
                        let mask = mask.trim_start_matches("*.");
                        file_name_str.ends_with(mask)
                    });

                if !matches_mask {
                    continue;
                }

                let is_component_selected =
                    configure
                        .components
                        .iter()
                        .filter(|c| c.selected)
                        .any(|component| {
                            crate::service::is_match(
                                &path,
                                &component.name,
                                &configure.extension_mask,
                            )
                        });

                if is_component_selected {
                    files_to_copy.push((path, file_name_str));
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

            let copy_stream = stream::iter(files_to_copy).map(|(path, file_name)| {
                let tx = tx.clone();
                let done_counter = Arc::clone(&done_counter);
                let to = Path::new(&dest_path_clone).join(&file_name);

                async move {
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
