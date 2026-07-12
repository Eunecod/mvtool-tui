// libs/mvframe/src/lib.rs

//                           |
//            _           _  | [esud] mvframe
//  _____ _ _| |_ ___ ___| | | 30/01/2026
// |     | | |  _| . | . | | |
// |_|_|_|\_/|_| |___|___|_| | Лицензия: MIT / Apache 2.0
//                           |

pub mod backend;
pub use backend::RenderContext;
pub use backend::UiState;

pub mod widget;
pub use widget::ComponentsWidget;
pub use widget::ConfiguresWidget;
pub use widget::ConsoleWidget;
pub use widget::ProjectsWidget;
