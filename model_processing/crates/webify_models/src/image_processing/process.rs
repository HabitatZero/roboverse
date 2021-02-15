//! Orchestrator to convert texture images from whatever format they're in to PNG

use std::path::Path;

use crate::cli;
use crate::image_processing;

/// Orchestrator to convert texture images from whatever format they're in to PNG
pub fn process(dir: &Path) -> std::result::Result<(), std::io::Error> {
  let images = image_processing::scan_dir_for_images(dir).unwrap();
  let image_bar = cli::create_progress_bar(images.len() as u64);

  for image in images {
    image_bar.inc(1);
    let moved_image = image_processing::move_to_textures_dir(image, dir, &image_bar).unwrap();
    image_processing::convert_to_png(moved_image, &image_bar).unwrap();
  }
  image_bar.finish_with_message("Images webified!");

  Ok(())
}

#[cfg(test)]
mod process_tests {
  use super::*;

  #[test]
  fn it_processes_the_dir() {
    assert!(false);
  }
}
