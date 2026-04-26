// source/widgets/helper/waiter.rs

#[derive(Clone, Copy)]
pub struct WaiterState {
    pub tick_count: u32,
    pub process: bool,
}

pub struct WaiterWidget;

impl WaiterWidget {
    const TICK_DIVIDER: usize = 15;
    const FRAMES: [&str; 6] = ["◜", "◠", "◝", "◞", "◡", "◟"];

    pub fn get_frame(state: &mut WaiterState) -> &str {
        if !state.process {
            return "";
        }

        let slowed_tick = (state.tick_count as usize) / Self::TICK_DIVIDER;
        let index = slowed_tick % Self::FRAMES.len();

        Self::FRAMES[index]
    }

    pub fn tick(state: &mut WaiterState) {
        if !state.process {
            state.tick_count = 0;
            return;
        }

        state.tick_count = state.tick_count.wrapping_add(1);
    }
}