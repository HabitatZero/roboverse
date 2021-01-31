use std::{fs, path::PathBuf};
use std::path::Path;

use console::style;

pub fn scan_dir_for_meshes(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
  println!("\nScanning for meshes to webify...");
  let mut meshes = match recursive_scan(dir, Vec::new()) {
    Ok(mesh_list) => mesh_list,
    Err(error) => panic!("Failed to scan all directories for meshes: {:?}", error),
  };

  println!("Meshes found: {}\n", style(meshes.len()).bold().blue());

  Ok(meshes)
}

fn recursive_scan(dir: &Path, mut meshes: Vec<PathBuf>) -> std::io::Result<Vec<PathBuf>> {
  if dir.is_dir() {
      for entry in fs::read_dir(dir)? {
          let e = entry?;
          let path = e.path();

          if path.is_dir() {
              meshes = recursive_scan(&path, meshes.clone())?;
          } else {
              let extension = match path.extension() {
                  Some(ext) => ext.to_str().unwrap(),
                  _ => ""
              };

              if extension == "dae" { meshes.push(path); };
          }
      }
  }

  Ok(meshes)
}
