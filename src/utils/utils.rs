// src/utils/utils.rs

use std::path::Path;

pub struct Utils;

impl Utils
{
    pub fn is_match(path: &Path, target_name: &str, extension_mask: &[String]) -> bool
    {
        let file_stem: std::borrow::Cow<'_, str> = path.file_stem().unwrap_or_default().to_string_lossy();
        let extension: std::borrow::Cow<'_, str> = path.extension().unwrap_or_default().to_string_lossy();

        return target_name == file_stem.as_ref() && (extension_mask.is_empty() || extension_mask.iter().any(|ext| ext == extension.as_ref()));
    }

    pub fn get_bit_depth() -> String
    {
        return if cfg!(target_pointer_width = "64") { "64-bit".into() } else { "32-bit".into() };
    }
}