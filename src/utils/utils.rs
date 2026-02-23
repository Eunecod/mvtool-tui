// src/utils/utils.rs

pub struct Utils;

impl Utils
{
    pub fn get_search_pattern(name: &str, prefix: &str, is_exception: bool) -> String
    {
        if is_exception || prefix.is_empty()
        {
            return name.to_string();
        }
        else
        {
            return format!("{}{}", name, prefix);
        }
    }
}