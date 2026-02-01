// utils/utils.rs

pub struct Utils; 

impl Utils
{
    pub fn add_prefix_before_ext(filename: &String, prefix: &String) -> String
    {
        let dot_position: Option<usize> = filename.rfind('.');
        if dot_position.is_none()
        {
            return "".to_string();
        }
        
        let (name, ext) = filename.split_at(dot_position.unwrap());
        return format!("{}{}{}", name, prefix, ext);
    } 
}