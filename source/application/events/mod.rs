// source/application/events/mod.rs

use crate::widgets::console::MessageType;
use crate::widgets::messagebox::SimpleMessageBox;
use crate::widgets::helper::waiter::WaiterState;
use crate::widgets::helper::toast::Toast;

pub enum AppEvent {
    Devent(String, MessageType),
    WaitProcess(WaiterState),
    ShowToast(Toast),
    ShowMessageBox(SimpleMessageBox),
}