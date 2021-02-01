use std::fs;
use std::path::PathBuf;

use aho_corasick::AhoCorasickBuilder;

pub fn rename_image_references(mesh: &PathBuf) -> std::result::Result<(), std::io::Error> {
  let result = find_and_rename_image_references(mesh)?;
  let final_result = update_texture_path(result)?;
  fs::write(mesh, final_result)?;

  Ok(())
}

fn find_and_rename_image_references(mesh: &PathBuf) -> std::result::Result<String, std::io::Error> {
  let patterns = &[".jpg", "_tga"];
  let f = fs::read_to_string(mesh)?;

  let ac = AhoCorasickBuilder::new().build(patterns);
  let result = ac.replace_all(&f, &[".png", "_png"]);

  Ok(result)
}

fn update_texture_path(contents: String) -> std::result::Result<String, std::io::Error> {
  // Find all image references (with PNG)
  // Get right relative directory path
  // Prefix image reference with relative directory path to textures
  // Call it a day (and return the string to the caller)

  Ok(contents)
}
