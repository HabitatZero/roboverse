use std::fs;

use image::io::Reader as ImageReader;
use image::ImageFormat::Tiff;

use console::style;
use indicatif::ProgressBar;

use super::image::Image;

pub fn convert_to_png(mut image: Image, progress_bar: &ProgressBar) -> std::io::Result<Image> {
  progress_bar.set_prefix("PNG Conversion");
  let styled_path = style(image.path.to_string_lossy()).dim();
  if image.extension == "png" {
    progress_bar.set_message(&format!("{} already in PNG, skipping", styled_path));
    return Ok(image);
  }

  progress_bar.set_message(&format!("Converting {}...", styled_path));
  let image_reader = match ImageReader::open(image.path.clone()) {
    Ok(img) => img,
    Err(e) => panic!("Failed to open image during PNG conversion: {:?}", e),
  };

  // Somehow, Tiff conversion is problematic, so we'll skip that
  if image_reader.format().is_some() && image_reader.format() != Some(Tiff) {
    let img = match image_reader.decode() {
      Ok(i) => i,
      Err(e) => panic!("Failed to open image during PNG conversion: {:?}", e),
    };

    match img.save(image.path.with_extension("png")) {
      Ok(_) => "",
      Err(e) => panic!("Could not convert {:?} to PNG: {:?}", image.path, e),
    };

    fs::remove_file(&image.path)?;
    progress_bar.set_message(&format!("{} converted!", styled_path));
    image.path = image.path.with_extension("png");
  } else {
    progress_bar.set_message(&format!(
      "{} skipped, as conversion of this file is problematic at this moment.",
      styled_path
    ));
  }

  Ok(image)
}
