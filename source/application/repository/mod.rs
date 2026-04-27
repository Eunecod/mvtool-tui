// source/application/repository/mod.rs

use std::fs;
use std::path::PathBuf;

use crate::models::Root;

pub trait Repository {
    fn load(&self) -> Result<Root, String>;

    #[expect(unused)]
    fn save(&self, settings: &Root) -> Result<(), String>;
}

pub struct JsonRepository {
    path: PathBuf,
}

impl JsonRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl Repository for JsonRepository {
    fn load(&self) -> Result<Root, String> {
        let content = fs::read_to_string(&self.path).map_err(|error| format!("Ошибка при загрузке setting.json: {}", error))?;

        serde_json::from_str(&content).map_err(|error| format!("Ошибка чтения JSON файла: {}", error))
    }

    fn save(&self, _settings: &Root) -> Result<(), String> {
        Ok(())
    }
}