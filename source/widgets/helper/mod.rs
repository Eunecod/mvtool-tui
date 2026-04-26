// source/widgets/helper/mod.rs

pub trait Item {
    fn name(&self) -> &str;
    fn selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
}

pub trait Executor {
    fn name(&self) -> &str;
    fn command(&self) -> &str ;
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct SelectionContext {
    pub context: Vec<usize>
}

pub mod list;
pub mod menu;
pub mod waiter;
pub mod toast;
pub mod action;