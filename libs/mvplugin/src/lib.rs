// libs/mvplugin/src/lib.rs

//                           |
//            _           _  | [esud] mvplugin
//  _____ _ _| |_ ___ ___| | | 30/01/2026
// |     | | |  _| . | . | | |
// |_|_|_|\_/|_| |___|___|_| | Лицензия: MIT / Apache 2.0
//                           |

pub mod api;
pub mod system;

use tokio::sync::mpsc;

use mvcore::events::Command;

pub struct Api {
    pub tx: mpsc::Sender<Command>,
}
