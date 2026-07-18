// libs/updater/src/lib.rs

//                           |
//            _           _  | [esud] updater
//  _____ _ _| |_ ___ ___| | | 30/01/2026
// |     | | |  _| . | . | | |
// |_|_|_|\_/|_| |___|___|_| | Лицензия: MIT / Apache 2.0
//                           |

pub mod net;
use net::github::Release;

use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

use mvcore::events::Command;
use mvcore::events::Type;

use std::fs::File as StdFile;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use walkdir::WalkDir;

use reqwest::Client;
use reqwest::header::USER_AGENT;

const BINARYNAME: &str = "mvtool";

pub struct Updater {
    client: Client,
    release: Release,
}

impl Updater {
    pub async fn new(url_repository: &str) -> Result<Self, String> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|error| format!("Ошибка создания клиента обновления: {}", error))?;

        let release: Release = client
            .get(url_repository)
            .header(USER_AGENT, BINARYNAME)
            .send()
            .await
            .map_err(|error| format!("Ошибка отправки запроса на обновления: {}", error))?
            .json()
            .await
            .map_err(|error| format!("Ошибка парсинга JSON релиза: {}", error))?;

        Ok(Self { client, release })
    }

    pub async fn update(
        &self,
        tx: mpsc::Sender<Command>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let target = Self::target_triple();

        let asset = self
            .release
            .assets
            .iter()
            .find(|asset| asset.name.contains(&target))
            .ok_or("Подходящего актива для обновления не найдено")?;

        let updates_dir = Path::new("updates");
        tokio::fs::create_dir_all(updates_dir).await?;

        let archive_path = updates_dir.join(&asset.name);

        let mut resp = self
            .client
            .get(&asset.browser_download_url)
            .header(USER_AGENT, BINARYNAME)
            .send()
            .await?;

        let mut file = TokioFile::create(&archive_path).await?;

        let mut downloaded = 0;
        let mut last_percent = 0;

        let _ = tx
            .send(Command::Devent(
                format!("Загрузка: {}", asset.name),
                Type::Message,
            ))
            .await;

        while let Some(chunk_result) = resp.chunk().await? {
            file.write_all(&chunk_result).await?;

            downloaded += chunk_result.len() as u64;
            if asset.size > 0 {
                let percent = ((downloaded as f64 / asset.size as f64) * 100.0) as u32;

                if percent != last_percent {
                    last_percent = percent;

                    let _ = tx
                        .send(Command::Devent(
                            format!("Прогресс: {}%", percent),
                            Type::Message,
                        ))
                        .await;
                }
            }
        }

        file.flush().await?;

        let extract_path = updates_dir.join("extracted");
        tokio::fs::create_dir_all(&extract_path).await?;

        let _ = tx
            .send(Command::Devent(
                "Распаковка архива...".into(),
                Type::Message,
            ))
            .await;

        let archive_path_clone = archive_path.clone();
        let extract_path_clone = extract_path.clone();

        tokio::task::spawn_blocking(move || -> Result<(), String> {
            let file = StdFile::open(&archive_path_clone)
                .map_err(|error| format!("Не удалось открыть архив: {}", error))?;
            let mut archive = tar::Archive::new(file);
            archive
                .unpack(&extract_path_clone)
                .map_err(|error| format!("Ошибка распаковки: {}", error))?;
            Ok(())
        })
        .await
        .map_err(|error| format!("Ошибка задачи распаковки: {}", error))??;

        let extract_path_clone = extract_path.clone();
        let new_bin =
            tokio::task::spawn_blocking(move || Self::find_binary_sync(&extract_path_clone))
                .await
                .map_err(|error| format!("Ошибка задачи поиска: {}", error))??;

        let updates_dir_clone = updates_dir.to_path_buf();
        tokio::task::spawn_blocking(move || -> Result<(), String> {
            self_replace::self_replace(&new_bin)
                .map_err(|error| format!("Ошибка замены файла: {}", error))?;

            let _ = std::fs::remove_dir_all(updates_dir_clone);
            Ok(())
        })
        .await
        .map_err(|error| format!("Ошибка задачи замены: {}", error))??;

        let _ = tx
            .send(Command::Devent(
                "Обновление завершено".into(),
                Type::Success,
            ))
            .await;

        Ok(())
    }

    pub fn get_latest_version(&self) -> &str {
        self.release.tag_name.as_str()
    }

    pub fn get_release_link(&self) -> &str {
        self.release.html_url.as_str()
    }

    pub fn is_update_available(&self) -> bool {
        mvcore::service::is_newer_version(self.release.tag_name.trim_start_matches('v'))
    }

    fn target_triple() -> String {
        format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
    }

    fn find_binary_sync(dir: &Path) -> Result<PathBuf, String> {
        for entry in WalkDir::new(dir) {
            let entry = entry.map_err(|error| format!("Ошибка обхода директории: {}", error))?;

            if entry.file_type().is_file() {
                let name = entry.file_name().to_string_lossy();

                if name == BINARYNAME || name.ends_with(".exe") {
                    return Ok(entry.path().to_path_buf());
                }
            }
        }

        Err("Двоичный файл для замены не найден".into())
    }
}
