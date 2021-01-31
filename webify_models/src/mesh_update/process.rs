use std::path::Path;

use crate::cli;
use crate::mesh_update;

pub fn process(dir: &Path) -> std::result::Result<(), std::io::Error> {
  let meshes = mesh_update::scan_dir_for_meshes(dir).unwrap();
  let mesh_bar = cli::create_progress_bar(meshes.len() as u64);

  for mesh in meshes {
    mesh_bar.inc(1);
  }
  mesh_bar.finish_with_message("Meshes webified!");

  Ok(())
}
