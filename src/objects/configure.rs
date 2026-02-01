// objects/configure.rs

#[derive(Debug, Default)]
pub struct Configure
{
    name: String,
    prefix: String,

    selected: bool,
}

impl Configure
{
    pub fn new(name: String, prefix: String, selected: bool) -> Self
    {
        return Self { name: name, prefix: prefix, selected: selected };
    }

    pub fn get_name(&self) -> &String
    {
        return &self.name;
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
}