// apps/mvtool/main.rs

//                           |
//            _           _  | [esud] mvtool
//  _____ _ _| |_ ___ ___| | | 30/01/2026
// |     | | |  _| . | . | | |
// |_|_|_|\_/|_| |___|___|_| | Лицензия: MIT / Apache 2.0
//                           |

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mvtool;
use mvtool::Application;

mod domain;

fn main() {
    let mut app = Application::bootstrap();
    app.run();
}
