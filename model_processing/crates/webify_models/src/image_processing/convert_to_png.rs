//! Converts an image file to PNG, or skips if it's already PNG

use std::{fs, io::Error, panic, result::Result};

use image::io::Reader as ImageReader;
use image::ImageFormat::Tiff;

use crate::image_processing::Image;

/// Convert the specified image to a PNG version
pub fn convert_to_png(mut image: Image) -> Result<Image, Error> {
  if image.extension == "tif" {
    return Ok(image); // Skip tif!
  }
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
  } else {
    panic!("Failed to convert provided image: {:?}", image.path);
  }

  Ok(image)
}

#[cfg(test)]
mod convert_tests {
  use super::*;

  use std::path::Path;

  fn setup(test_run_id: &str) -> Result<(), Error> {
    let example_image_path = Path::new("tests").join("image_processing").join("images");
    let destination_path = Path::new("tests")
      .join("image_processing")
      .join(test_run_id);
    fs::create_dir_all(&destination_path)?;
    fs::copy(
      example_image_path.join("example.jpg"),
      &destination_path.join("example.jpg"),
    )?;
    fs::copy(
      example_image_path.join("README.md"),
      &destination_path.join("README.md"),
    )?;

    Ok(())
  }

  fn teardown(test_run_id: &str) -> Result<(), Error> {
    let destination_path = Path::new("tests")
      .join("image_processing")
      .join(test_run_id);
    fs::remove_dir_all(destination_path)?;

    Ok(())
  }

  #[test]
  fn it_converts_a_jpg_to_png() -> Result<(), Error> {
    let test_run_name = "test_run_it_converts_a_jpg_to_png";
    setup(test_run_name)?;

    let test_image_path = Path::new("tests")
      .join("image_processing")
      .join(test_run_name)
      .join("example.jpg");
    assert!(Path::exists(&test_image_path));

    let image = Image {
      path: test_image_path,
      extension: String::from("jpg"),
    };

    convert_to_png(image)?;
    // Check that previous test image was deleted
    assert!(!Path::exists(
      &Path::new("tests")
        .join("image_processing")
        .join(test_run_name)
        .join("example.jpg")
    ));
    // Check that new image is there as a PNG
    assert!(Path::exists(
      &Path::new("tests")
        .join("image_processing")
        .join(test_run_name)
        .join("example.png")
    ));

    teardown(test_run_name)?;
    Ok(())
  }

  #[test]
  fn it_panics_on_non_images() {
    let test_run_name = "test_run_it_panics_on_non_images";
    setup(test_run_name).unwrap();

    let test_image_path = Path::new("tests")
      .join("image_processing")
      .join(test_run_name)
      .join("README.md");
    assert!(Path::exists(&test_image_path));

    let non_image = Image {
      path: test_image_path,
      extension: String::from("jpg"),
    };

    // Catch the panic here so we can teardown after, otherwise
    // just using should_panic will leave use with no teardown
    let result = panic::catch_unwind(|| {
      convert_to_png(non_image).unwrap();
    });
    assert!(result.is_err());

    teardown(test_run_name).unwrap();
  }
}
