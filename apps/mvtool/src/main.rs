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

#[cfg(target_os = "windows")]
unsafe extern "system" {
    fn AttachConsole(dw_process_id: u32) -> i32;
    fn FreeConsole() -> i32;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--version".into()) || args.contains(&"-v".into()) {
        #[cfg(target_os = "windows")]
        unsafe {
            AttachConsole(0xFFFF_FFFF);
        }

        use std::io::Write;

        println!("\nmvtool version: {}", mvcore::service::version());
        let _ = std::io::stdout().flush();

        #[cfg(target_os = "windows")]
        unsafe {
            FreeConsole();
        }
        std::process::exit(0);
    }

    let mut app = Application::bootstrap();
    app.run();
}
