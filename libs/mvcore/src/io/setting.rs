// libs/mvcore/src/io/settings.rs

pub trait Setting {
    fn read(&mut self, key: &str, value: &str);
    fn write(&self, buffer: &mut String);
}

pub struct ApplicationSetting {
    pub url_repository: String,
}

impl Default for ApplicationSetting {
    fn default() -> Self {
        Self {
            url_repository: String::new(),
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
            _ => {}
        }
    }

    fn write(&self, buffer: &mut String) {
        buffer.push_str(&format!("url_repository={}\n", self.url_repository));
    }
}
