// src/objects/script.rs

#[derive(Debug, Default)]
pub struct Script
{
    name: String,
    command: String,
}

impl Script
{
    pub fn new(name: String, command: String) -> Self
    {
        return Self { name, command };
    }

    pub fn get_name(&self) -> &String
    {
        return &self.name;
    }

    pub fn get_command(&self) -> &String
    {
        return &self.command;
    }
}