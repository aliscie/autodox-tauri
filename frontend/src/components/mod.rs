// here we create general specific_components that are reusable by anyapp
// Don't import anything this folder from outside.

pub use avatar::Avatar;
pub use directory::CurrDirectory;

pub use menu::ContextMenu;
pub use title_bar::TitleBar;
pub use tree_list::TreeList;

pub mod title_bar;

mod tree_list;

mod avatar;
mod directory;
mod menu;
