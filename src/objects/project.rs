// objects/project.rs

use crate::Component;

#[derive(Debug, Default)]
pub struct Project
{
    name: String,
    path: String,
    components: Vec<Component>,
    selected: bool,
}

impl Project
{
    pub fn new(name: String, path: String, components: Vec<Component>, selected: bool) -> Self
    {
        return Self { name: name, path: path, components: components, selected: selected };
    }

    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }

    pub fn get_path(&self) -> &String
    {
        return &self.path;
    }

    pub fn get_components_mut(&mut self) -> &mut Vec<Component>
    {
        return &mut self.components;
    }

    pub fn get_components(&self) -> &Vec<Component>
    {
        return &self.components;
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