//! Orchestrator to run the mesh updater

use std::path::Path;

use crate::cli;
use crate::mesh_update;

/// Orchestrator to run the mesh updater
pub fn process(dir: &Path) -> std::result::Result<(), std::io::Error> {
  let meshes = mesh_update::scan_dir_for_meshes(dir).unwrap();
  let mesh_bar = cli::create_progress_bar(meshes.len() as u64);

  mesh_bar.set_prefix("Mesh Update");
  for mesh in meshes {
    mesh_bar.inc(1);
    mesh_bar.set_message(&format!("Updating {:?}...", &mesh));
    mesh_update::rename_image_references(&mesh)?;
  }

  // TODO: Update image references in material, txt, and sdf

  mesh_bar.finish_with_message("Meshes webified!");

  Ok(())
}

#[cfg(test)]
mod process_tests {

  #[test]
  #[ignore = "not yet implemented"]
  fn it_processes_the_dir() {
    assert!(false);
  }
}
