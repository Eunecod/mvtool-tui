// libs/core/src/service.rs

use std::path::Path;

use image::RgbaImage;

use notify_rust::Notification;
use notify_rust::Timeout;

use qrcode::QrCode;
use qrcode::render::unicode;

#[cfg(windows)]
use winreg::RegKey;
#[cfg(windows)]
use winreg::enums::HKEY_CURRENT_USER;

#[cfg(windows)]
const AUMID_PATH: &str = r"SOFTWARE\Classes\AppUserModelId";
#[cfg(windows)]
const NOTIFICATION_AUMID: &str = "com.mvtool.desktop";

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

pub fn load_icon(raw_data: &[u8]) -> RgbaImage {
    image::load_from_memory(raw_data)
        .map(|buffer| buffer.into_rgba8())
        .unwrap_or_else(|_| RgbaImage::new(16, 16))
}

#[cfg(windows)]
pub fn register_aumid(display_name: &str) -> Result<(), String> {
    let (key, _) = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(format!(r"{}\{}", AUMID_PATH, NOTIFICATION_AUMID))
        .map_err(|error| error.to_string())?;

    key.set_value("DisplayName", &display_name)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(windows)]
pub fn is_registered_aumid() -> bool {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(&format!(r"{}\{}", AUMID_PATH, NOTIFICATION_AUMID))
        .is_ok()
}

pub fn show_notification(appname: &str, title: &str, message: &str) {
    let mut notification = Notification::new();
    notification.appname(appname);
    #[cfg(windows)]
    {
        if is_registered_aumid() {
            notification.app_id(NOTIFICATION_AUMID);
        }
    }

    notification
        .summary(title)
        .body(message)
        .timeout(Timeout::Default)
        .auto_icon();

    let _ = notification.show();
}

pub fn bit_depth() -> String {
    return if cfg!(target_pointer_width = "64") {
        "64-bit".into()
    } else {
        "32-bit".into()
    };
}

pub fn qrcode(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    image
}

pub fn open_url(url: &str) {
    open::that(url).ok();
}

pub fn is_match(path: &Path, target_name: &str, extension_mask: &[String]) -> bool {
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    if target_name != file_stem {
        return false;
    }

    if extension_mask.is_empty() {
        return true;
    }

    extension_mask
        .iter()
        .any(|mask| mask.trim_start_matches("*.") == extension)
}
