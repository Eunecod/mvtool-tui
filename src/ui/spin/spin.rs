// src/ui/spin/spin.rs

use std::sync::mpsc;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Spin
{
    pub state: SpinState,
    pub sender: mpsc::Sender<SpinState>,
    pub receiver: mpsc::Receiver<SpinState>
}

impl Spin
{
    const FRAMES: &'static [&'static str] = &["◜", "◠", "◝", "◞", "◡", "◟"];

    pub fn new(state: SpinState) -> Self
    {
        let (sender, receiver) = mpsc::channel();  
        return Self { state, sender, receiver };
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

    pub fn get_sender(&self) -> mpsc::Sender<SpinState>
    {
        self.sender.clone()
    }

    pub fn update(&mut self)
    {
        if self.state.procces
        {
            self.state.tick_count = self.state.tick_count.wrapping_add(1);
        }
        else
        {
            self.state.tick_count = 0;
        }

        while let Ok(event) = self.receiver.try_recv()
        {
            self.state = event;
        }
    }
}