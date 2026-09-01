// libs/core/src/service.rs

use std::fs::File as StdFile;
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
    let file_name = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return false,
    };

    let matches_name = if file_name.starts_with(target_name) {
        let remainder = &file_name[target_name.len()..];
        remainder.is_empty() || remainder.starts_with('.')
    } else {
        false
    };

    if !matches_name {
        return false;
    }

    if extension_mask.is_empty() {
        return true;
    }

    extension_mask.iter().any(|mask| {
        let ext = mask.trim_start_matches('*');
        file_name.ends_with(ext)
    })
}

pub fn is_newer_version(target_version: &str) -> bool {
    if let (Ok(current), Ok(target)) = (
        semver::Version::parse(&version()),
        semver::Version::parse(target_version),
    ) {
        return target > current;
    }

    false
}

pub fn unpackage_tar(archive_path: &Path, extract_path: &Path) -> Result<(), String> {
    let file = StdFile::open(&archive_path)
        .map_err(|error| format!("Не удалось открыть архив: {}", error))?;
    let mut archive = tar::Archive::new(file);
    archive
        .unpack(&extract_path)
        .map_err(|error| format!("Ошибка распаковки: {}", error))?;

    Ok(())
}
