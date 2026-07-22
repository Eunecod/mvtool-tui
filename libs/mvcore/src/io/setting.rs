// libs/mvcore/src/io/settings.rs

pub trait Setting {
    fn read(&mut self, key: &str, value: &str);
    fn write(&self, buffer: &mut String);
}

pub struct ApplicationSetting {
    pub url_repository: String,
    pub try_update: bool,
}

impl Default for ApplicationSetting {
    fn default() -> Self {
        Self {
            url_repository: String::new(),
            try_update: true,
        }
    }
}

impl Setting for ApplicationSetting {
    fn read(&mut self, key: &str, value: &str) {
        match key {
            "url_repository" => {
                value
                    .parse::<String>()
                    .ok()
                    .map(|url_repository| self.url_repository = url_repository);
            }
            "try_update" => {
                value
                    .parse::<bool>()
                    .ok()
                    .map(|try_update| self.try_update = try_update);
            }
            _ => {}
        }
    }

    fn write(&self, buffer: &mut String) {
        buffer.push_str(&format!("url_repository={}\n", self.url_repository));
        buffer.push_str(&format!("try_update={}\n", self.try_update));
    }
}
