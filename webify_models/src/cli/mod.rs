//! Set of functions related to the command-line interface for webify_models

mod create_progress_bar;
mod parse_args_for_path;

pub use self::create_progress_bar::create_progress_bar;
pub use self::parse_args_for_path::parse_args_for_path;
