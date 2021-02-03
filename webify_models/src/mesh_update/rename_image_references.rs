//! Rename all image references inside a DAE mesh to be .PNG and point
//! at the right textures path

use std::{fs::{self}, path::Path};
use std::path::PathBuf;

use aho_corasick::AhoCorasickBuilder;

/// Orchestrator to rename image references in a DAE mesh
pub fn rename_image_references(mesh: &PathBuf) -> std::result::Result<(), std::io::Error> {
  let result = find_and_rename_image_references(mesh)?;
  let final_result = update_texture_path(result)?;
  fs::write(mesh, final_result)?;

  Ok(())
}

/// Rename all occurences of supported image types to PNG
fn find_and_rename_image_references(mesh: &PathBuf) -> std::result::Result<String, std::io::Error> {
  let patterns = &[".jpg", "_tga"];
  let f = fs::read_to_string(mesh)?;

  let ac = AhoCorasickBuilder::new().build(patterns);
  let result = ac.replace_all(&f, &[".png", "_png"]);

  Ok(result)
}

/// Update all texture paths to point to the relative path for textures/materials
fn update_texture_path(contents: String) -> std::result::Result<String, std::io::Error> {
  let texture_element = &["<init_from>", "</init_from>"];
  let ac = AhoCorasickBuilder::new().build(texture_element);
  let texture_path = Path::new("materials").join("textures");

  let mut new_contents = String::new();

  // Find all texture elements
  for line in contents.lines() {
    let mut start = 0;
    let mut new_line: String = String::new();
    for mat in ac.find_iter(&line) {
      if mat.pattern() == 0 {
        start = mat.end();
      } else if mat.pattern() == 1 {
        let texture_name = &line[start..mat.start()];
        // Prefix image reference with relative directory path to textures
        if texture_name.ends_with(".png") && !texture_name.contains(&texture_path.to_str().unwrap()) {
          // TODO: Properly find the root path of the mesh, rather than assuming
          new_line = line.replace(texture_name, Path::new("..").join("materials").join("textures").join(texture_name).to_str().unwrap());
        }
      }
    }
    // TODO: Fix the output of this to keep the newlines
    if new_line.is_empty() {
      new_contents.push_str(format!("{}\n", line).as_str());
    } else {
      new_contents.push_str(format!("{}\n", new_line).as_str());
    }

  };

  Ok(new_contents)
}
