// src/ui/spin/spin.rs

pub struct SpinState
{
    pub tick_count: u32,
    pub procces: bool,
}

impl SpinState
{
    pub fn new(tick_count: u32, procces: bool) -> Self
    {
        return Self { tick_count, procces };
    }
}

pub struct Spin
{
    pub state: SpinState,
}

impl Spin
{
    const FRAMES: &'static [&'static str] = &["⠁", "⠉", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠃"];

    pub fn new(state: SpinState) -> Self
    {
        return Self { state };
    }

    pub fn get_frame(&self) -> &str
    {
        if !self.state.procces
        {
            return "";
        }

        let index: usize = (self.state.tick_count as usize) % Self::FRAMES.len();
        return Self::FRAMES[index];
    }

    pub fn tick(&mut self)
    {
        if !self.state.procces
        {
            self.state.tick_count = 0;
            return;
        }

        self.state.tick_count = self.state.tick_count.wrapping_add(1);
    }
}