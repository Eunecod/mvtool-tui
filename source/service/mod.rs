// source/service/mod.rs

use std::path::Path;

use qrcode::QrCode;
use qrcode::render::unicode;

#[cfg(windows)]
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn is_match(path: &Path, target_name: &str, extension_mask: &[String]) -> bool
{
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    if target_name != file_stem {
        return false;
    }

    if extension_mask.is_empty() {
        return true;
    }

    extension_mask.iter().any(|mask| mask.trim_start_matches("*.") == extension)
}

pub fn bit_depth() -> String {
    return if cfg!(target_pointer_width = "64") { "64-bit".into() } else { "32-bit".into() };
}

pub fn qrcode(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    image
}

#[cfg(windows)]
pub fn register_aumid(aumid: &str, display_name: &str) -> Result<(), String> {
    let (key, _) = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(format!(r"SOFTWARE\Classes\AppUserModelId\{}", aumid))
        .map_err(|error| error.to_string())?;

    key.set_value("DisplayName", &display_name).map_err(|error| error.to_string())?;

    Ok(())
}

#[cfg(windows)]
pub fn is_registered_aumid(aumid: &str) -> bool {
    RegKey::predef(HKEY_CURRENT_USER).open_subkey(&format!(r"Software\Classes\AppUserModelId\{}", aumid)).is_ok()
}