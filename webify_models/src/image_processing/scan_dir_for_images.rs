use console::style;
use std::{fs, path::Path};

use super::image::Image;

const TEXTURE_IMAGE_TYPES: [&str; 7] = [
  r#"tif"#, r#"tga"#, r#"tiff"#, r#"jpeg"#, r#"jpg"#, r#"gif"#, r#"png"#,
];

pub fn scan_dir_for_images(dir: &Path) -> std::io::Result<Vec<Image>> {
  println!("\nScanning for images to webify...");
  println!(
    "{}",
    &style(
      "Note that webify models is a destructive action and will DELETE the existing non-PNG files."
    )
    .on_red()
  );

  let mut images = match recursive_scan(dir, Vec::new()) {
    Ok(image_list) => image_list,
    Err(error) => panic!("Failed to scan all directories for images: {:?}", error),
  };
  images.sort_by(|a, b| b.extension.cmp(&a.extension));

  println!("Images found: {}\n", style(images.len()).bold().blue());

  Ok(images)
}

fn recursive_scan(dir: &Path, mut images: Vec<Image>) -> std::io::Result<Vec<Image>> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let e = entry?;
      let path = e.path();

      if path.is_dir() {
        images = recursive_scan(&path, images.clone())?;
      } else {
        let extension = match path.extension() {
          Some(ext) => ext.to_str().unwrap(),
          _ => "",
        };

        if TEXTURE_IMAGE_TYPES.contains(&extension) {
          images.push(Image {
            path: path.clone(),
            extension: extension.to_string(),
          });
        };
      }
    }
  }

  Ok(images)
}
