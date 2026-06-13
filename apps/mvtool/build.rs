// apps/mvtool/build.rs

//                           |
//            _           _  | [esud] mvtool
//  _____ _ _| |_ ___ ___| | | 13/06/2026
// |     | | |  _| . | . | | |
// |_|_|_|\_/|_| |___|___|_| | Лицензия: MIT / Apache 2.0
//                           |

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("src/platforms/windows/icon.ico")
            .set("ProductName", "mvtool")
            .set("ProductVersion", VERSION)
            .set(
                "LegalCopyright",
                "Copyright (c) 2026 esud. All rights reserved.",
            )
            .set("LegalTrademarks", "MIT/Apache 2.0");

        match res.compile() {
            Ok(_) => println!("compiled resources successfully mvtool v{}", VERSION),
            Err(error) => println!("cargo:warning=Failed to compile resources: {}", error),
        }
    }

    #[cfg(not(windows))]
    {
        println!("Skipping Windows resources (cross-platform build)");
    }
}
