// libs/mvframe/src/widget/settings/data.rs

use mvcore::io::ApplicationSetting;

pub struct SettingsData<'a> {
    pub application_setting: &'a mut ApplicationSetting,
}

impl<'a> SettingsData<'a> {
    pub fn new(application_setting: &'a mut ApplicationSetting) -> Self {
        Self {
            application_setting,
        }
    }
}
