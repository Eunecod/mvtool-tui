// src/objects/project.rs

use crate::objects::Configure;

#[derive(Default, Clone)]
pub struct Project
{
    name: String,
    destination_path: String,
    configures: Vec<Configure>,
    selected: bool,
}

impl Project
{
    pub fn new(name: String, destination_path: String, configures: Vec<Configure>, selected: bool) -> Self
    {
        return Self { name, destination_path, configures, selected };
    }

    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }

    pub fn get_destination(&self) -> &String
    { 
        return &self.destination_path;
    }

    pub fn get_configures(&self) -> &Vec<Configure>
    { 
        return &self.configures;
    }

    pub fn get_configures_mut(&mut self) -> &mut Vec<Configure>
    {
        return &mut self.configures;
    }

    pub fn is_selected(&self) -> bool
    {
        return self.selected;
    }

    pub fn set_selected(&mut self, selected: bool)
    {
        self.selected = selected;
    }
}