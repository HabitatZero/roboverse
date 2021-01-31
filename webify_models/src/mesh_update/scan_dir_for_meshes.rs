use std::{fs, path::PathBuf};
use std::path::Path;

pub fn scan_dir_for_meshes(dir: &Path, mut meshes: Vec<PathBuf>) -> std::io::Result<Vec<PathBuf>> {
  if dir.is_dir() {
      for entry in fs::read_dir(dir)? {
          let e = entry?;
          let path = e.path();

          if path.is_dir() {
              meshes = scan_dir_for_meshes(&path, meshes.clone())?;
          } else {
              let extension = match path.extension() {
                  Some(ext) => ext.to_str().unwrap(),
                  _ => ""
              };

              if extension == "dae" {
                  meshes.push(path);
              };
          }
      }
  }
  Ok(meshes)
}
