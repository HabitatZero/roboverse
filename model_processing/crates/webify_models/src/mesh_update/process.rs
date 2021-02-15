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
  use std::fs::{self, File};
  use std::io::prelude::*;

  use super::*;

  fn setup(test_run_id: &str) -> std::result::Result<(), std::io::Error> {
    let example_path = Path::new("tests").join("mesh_update").join("test");
    let destination_path = Path::new("tests").join("mesh_update").join(test_run_id);

    fs::create_dir_all(&destination_path.join("meshes"))?;
    fs::create_dir_all(&destination_path.join("materials").join("textures"))?;

    fs::copy(example_path.join("meshes").join("test.dae"), &destination_path.join("meshes").join("test.dae"))?;

    Ok(())
  }

  fn teardown(test_run_id: &str) -> std::result::Result<(), std::io::Error> {
    let destination_path = Path::new("tests").join("mesh_update").join(test_run_id);
    fs::remove_dir_all(destination_path)?;

    Ok(())
  }

  #[test]
  fn it_updated_the_mesh() -> std::result::Result<(), std::io::Error> {
    let test_run_id = "process_1";

    setup(&test_run_id)?;

    let destination_path = Path::new("tests").join("mesh_update").join(&test_run_id).join("meshes");
    // The fact that it outputs the progress bar, but Rust, how the heck do you stub/mock this?
    process(&destination_path)?;

    let mut file = File::open(destination_path.join("test.dae"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    assert_eq!(contents, "<!-- This is not a valid DAE, just a test file -->\n\n<image id=\"Test_Diffuse_png\">\n  <init_from>../materials/textures/test_diffuse.png</init_from>\n</image>\n");

    teardown(&test_run_id)?;

    Ok(())
  }
}
