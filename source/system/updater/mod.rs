// source/system/updater/mod.rs

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::mpsc::Sender;

use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

use serde::Deserialize;

use walkdir::WalkDir;

use crate::application::events::AppEvent;

use crate::widgets::console::MessageType;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,

    size: u64
}

pub struct Updater {
    bin_name: String,

    client: Client,
    release: Release
}

impl Updater {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bin_name = "mvtool".to_string();
        let url = "https://api.github.com/repos/Eunecod/mvtool-tui/releases/latest";

        let client = Client::builder().timeout(Duration::from_secs(15)).build()?;
        let release: Release = client.get(url).header(USER_AGENT, &bin_name).send()?.json()?;

        Ok(Self { bin_name, client, release })
    }

    pub fn get_latest_version(&self) -> &str {
        self.release.tag_name.as_str()
    }

    pub fn is_update_available(&self) -> bool {
        if let (Ok(current), Ok(new)) = (
            semver::Version::parse(env!("CARGO_PKG_VERSION")),
            semver::Version::parse(self.release.tag_name.trim_start_matches('v')),
        ) {
            return new > current;
        }

        false
    }

    pub fn update(&self, sender: Sender<AppEvent>) -> Result<(), Box<dyn std::error::Error>> {
        let target = Self::target_triple();

        let asset = self.release.assets.iter()
            .find(|asset| asset.name.contains(&target))
            .ok_or("Подходящего актива для обновления не найдено")?;

        let updates_dir = Path::new("updates");
        std::fs::create_dir_all(updates_dir)?;

        let archive_path = updates_dir.join(&asset.name);

        let mut resp = self.client
            .get(&asset.browser_download_url)
            .header(USER_AGENT, &self.bin_name)
            .send()?;

        let mut file = File::create(&archive_path)?;

        let mut buffer = [0u8; 8192];
        let mut downloaded = 0;
        let total_size = asset.size;
        let mut last_percent = 0;

        let _ = sender.send(AppEvent::Devent(format!("Загрузка: {}", asset.name), MessageType::Info));

        loop {
            let n = resp.read(&mut buffer)?;
            if n == 0 {
                break;
            }

            file.write_all(&buffer[..n])?;

            downloaded += n as u64;
            if total_size > 0 {
                let percent = ((downloaded as f64 / total_size as f64) * 100.0) as u32;

                if percent != last_percent {
                    last_percent = percent;

                    let _ = sender.send(AppEvent::Devent(format!("Прогресс: {}%", percent), MessageType::Info));
                }
            }
        }

        let extract_path = updates_dir.join("extracted");
        std::fs::create_dir_all(&extract_path)?;

        let _ = sender.send(AppEvent::Devent("Распаковка архива...".into(), MessageType::Info));

        let mut archive = tar::Archive::new(File::open(&archive_path)?);
        archive.unpack(&extract_path)?;

        let new_bin = self.find_binary(&extract_path)?;

        self_replace::self_replace(new_bin)?;

        let _ = std::fs::remove_dir_all(updates_dir);
        let _ = sender.send(AppEvent::Devent("Обновление завершено".into(), MessageType::Success));


        Ok(())
    }

    fn target_triple() -> String {
        format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
    }

    fn find_binary(&self, dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let name = entry.file_name().to_string_lossy();

                if name == self.bin_name || name.ends_with(".exe") {
                    return Ok(entry.path().to_path_buf());
                }
            }
        }

        Err("Двоичный файлов для замены не найдено".into())
    }
}