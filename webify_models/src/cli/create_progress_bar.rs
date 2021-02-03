//! Progress bar factory function, for all your progress bar needs

use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(item_count: u64) -> ProgressBar {
  let bar = ProgressBar::new(item_count);
  let bar_style: ProgressStyle = ProgressStyle::default_bar()
    .template(
      "{prefix:15.green.bold} {wide_msg}\n[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7}",
    )
    .progress_chars("█▓▒░");

  bar.set_style(bar_style.clone());

  bar
}
