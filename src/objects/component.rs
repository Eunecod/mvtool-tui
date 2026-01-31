// objects/component.rs

#[derive(Debug, Default)]
pub struct Component
{
    name: String,
    selected: bool,
}

impl Component
{
    pub fn new(name: String, selected: bool) -> Self
    {
        return Self { name: name, selected: selected };
    }

    pub fn get_name(&self) -> &String
    {
        return &self.name;
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