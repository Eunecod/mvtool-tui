// src/updater.rs

use reqwest::blocking::Client;
use serde_json::Value;
use std::io::{ Read, Write };
use std::fs::{ File };
use std::path::{ Path, PathBuf };
use std::sync::mpsc;

use crate::AppEvent;
use crate::ui::messagelog::MessageType;

pub struct ReleaseUpdateGithub
{
    pub version_current: String,
    pub version_new: String,
    pub is_available: bool,
}

impl Default for ReleaseUpdateGithub
{
    fn default() -> Self
    {
        return Self { version_current: String::default(), version_new: String::default(), is_available: false };
    }
}

impl ReleaseUpdateGithub
{
    pub fn new(versions: (String, String)) -> Self
    {
        let is_available: bool = versions.0 != versions.1;
        return Self { version_current: versions.0, version_new: versions.1, is_available: is_available, };
    }
}

pub struct Updater
{
    pub state: ReleaseUpdateGithub,

    owner: String,
    repo: String,
    bin_name: String,
}

impl Updater
{
    pub fn new() -> Self
    {
        return Self { state: ReleaseUpdateGithub::default(), owner: "Eunecod".to_string(), repo: "mvtool-tui".to_string(), bin_name: "mvtool".to_string(), };
    }

    pub fn fetch(mut self) -> Result<Self, Box<dyn std::error::Error>>
    {
        let client: Client   = Client::new();
        let response: String = client.get(format!("https://api.github.com/repos/{}/{}/releases/latest", self.owner, self.repo)).header("User-Agent", &self.bin_name).send()?.text()?;
        
        let json: Value = serde_json::from_str(&response)?;
        
        self.state = ReleaseUpdateGithub::new(
            (env!("CARGO_PKG_VERSION").to_string(), json["tag_name"].as_str().unwrap_or("0.0.0").trim_start_matches('v').to_string())
        );

        return Ok(self);
    }

    pub fn update(self, tx: mpsc::Sender<AppEvent>) -> Result<Self, Box<dyn std::error::Error>>
    {
        let client: Client   = Client::new();
        let response: String = client.get(format!("https://api.github.com/repos/{}/{}/releases/latest", self.owner, self.repo)).header("User-Agent", &self.bin_name).send()?.text()?;
        
        let json: Value = serde_json::from_str(&response)?;
        
        let mut browser_download_url: Option<String> = None;
        let mut asset_name: String = String::new();
        let mut total_size: u64 = 0u64;

        if let Some(assets) = json["assets"].as_array()
        {
            for asset in assets
            {
                let name: &str = asset["name"].as_str().unwrap_or("");
                if name.contains("x86_64-pc-windows-msvc")
                {
                    if !name.ends_with(".tar")
                    {
                        continue;
                    }

                    browser_download_url = asset["browser_download_url"].as_str().map(String::from);
                    asset_name = name.to_string();
                    total_size = asset["size"].as_u64().unwrap_or(0);

                    break;
                }
            }
        }
        
        let download_url: String = browser_download_url.ok_or("is not support platform")?;
        
        let updates_dir: &Path = Path::new("updates");
        std::fs::create_dir_all(updates_dir)?;
        let archive_path: PathBuf = updates_dir.join(&asset_name);
        
        let mut response: reqwest::blocking::Response = client.get(&download_url).header("User-Agent", &self.bin_name).send()?;
        
        let mut file: File = File::create(&archive_path)?;
        let mut downloaded: u64 = 0u64;
        let mut buffer: [u8; 16384] = [0; 16384];
        let mut last_reported_percent: u32 = 0u32;

        let _ = tx.send(AppEvent::Log(format!("downloading: {}", asset_name), MessageType::Info));

        loop
        {
            let bytes_read: usize = response.read(&mut buffer)?;
            if bytes_read == 0
            {
                break;
            }
            
            file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read as u64;

            if total_size > 0
            {
                let percent = ((downloaded as f64 / total_size as f64) * 100.0) as u32;  
                if percent > last_reported_percent
                {
                    last_reported_percent = percent;
                    let _ = tx.send(AppEvent::Log(format!("progress: {}%", percent), MessageType::Info));
                }
            }
        }

        let extract_path = updates_dir.join("extracted");
        std::fs::create_dir_all(&extract_path)?;
        
        let _ = tx.send(AppEvent::Log("extracting archive...".into(), MessageType::Info));
        let mut archive = tar::Archive::new(File::open(&archive_path)?);
        archive.unpack(&extract_path)?;

        let new_bin_path = self.find_binary(&extract_path)?;
        self_replace::self_replace(&new_bin_path)?;
        
        let _ = std::fs::remove_dir_all(updates_dir);
        
        return Ok(self);
    }
    
    fn find_binary(&self, dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>>
    {        
        fn search(dir: &Path, applicant_name: &str) -> Option<PathBuf>
        {
            if let Ok(entries) = std::fs::read_dir(dir)
            {
                for entry in entries.flatten()
                {
                    let path: PathBuf = entry.path();
                    if path.is_file()
                    {
                        if let Some(file_name) = path.file_name().and_then(|name| name.to_str())
                        {
                            if file_name == applicant_name || file_name.ends_with(".exe")
                            {
                                return Some(path);
                            }
                        }
                    }
                    else if path.is_dir()
                    {
                        if let Some(found) = search(&path, applicant_name)
                        {
                            return Some(found);
                        }
                    }
                }
            }

            return None;
        }
        
        let name: String = format!("{}.exe", self.bin_name);
        return search(dir, &name).ok_or_else(|| "application executable file was not found".into());
    }
}