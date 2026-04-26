// source/service/mod.rs

use std::path::Path;

#[cfg(windows)]
use register_app_for_toast::register;

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

#[cfg(windows)]
pub fn register_aumid(aumid: &str, display_name: &str) -> Result<(), String> {
    register(aumid, "D9E1B73A-4F8A-4B6E-9C3E-2A4D1F8C9E07", Some(display_name)).map_err(|error| error.to_string())?;

    Ok(())
}

// pub fn get_bit_depth() -> String
// {
//     return if cfg!(target_pointer_width = "64") { "64-bit".into() } else { "32-bit".into() };
// }