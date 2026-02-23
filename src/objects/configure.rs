// src/objects/configure.rs

use crate::objects::{ Component, Script };

#[derive(Debug, Default)]
pub struct Configure
{
    name: String,
    source_path: String,
    prefix: String,
    selected: bool,
    clean_destination: bool,
    components: Vec<Component>,
    scripts: Vec<Script>,
}

impl Configure
{
    pub fn new(name: String, source_path: String, prefix: String, selected: bool, clean_destination: bool, components: Vec<Component>, scripts: Vec<Script>) -> Self
    {
        return Self { name, source_path, prefix, selected, clean_destination, components, scripts };
    }

    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }

    pub fn get_path(&self) -> &String
    {
        return &self.source_path;
    }

    pub fn get_prefix(&self) -> &String
    {
        return &self.prefix;
    }

    pub fn is_selected(&self) -> bool
    {
        return self.selected;
    }

    pub fn set_selected(&mut self, selected: bool)
    {
        self.selected = selected;
    }

    pub fn should_clean(&self) -> bool
    {
        return self.clean_destination;
    }

    pub fn get_components(&self) -> &Vec<Component>
    {
        return &self.components; 
    }

    pub fn get_components_mut(&mut self) -> &mut Vec<Component>
    {
        return &mut self.components
    }

    pub fn get_scripts(&self) -> &Vec<Script>
    { 
        return &self.scripts;
    }
}