use std::fs;

use image::io::Reader as ImageReader;
use image::ImageFormat::Tiff;

use console::style;
use indicatif::ProgressBar;

use crate::image_processing::Image;

pub fn convert_to_png(
  image: Image,
  progress_bar: &ProgressBar,
) -> std::result::Result<Image, std::io::Error> {
  progress_bar.set_prefix("PNG Conversion");
  let styled_path = style(image.path.to_string_lossy()).dim();
  if image.extension == "png" {
    progress_bar.set_message(&format!("{} already in PNG, skipping", styled_path));
    return Ok(image);
  }

  progress_bar.set_message(&format!("Converting {}...", styled_path));
  let converted_image = convert(image.clone())?;
  progress_bar.set_message(&format!("{} converted!", styled_path));

  Ok(converted_image)
}

fn convert(mut image: Image) -> std::result::Result<Image, std::io::Error> {
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
    image.path = image.path.with_extension("png");
  }

  Ok(image)
}
