//! Orchestrator to run the mesh updater

use std::path::Path;

use crate::cli::create_progress_bar;
use crate::mesh_update::{rename_image_references, scan_dir_for_meshes};

/// Orchestrator to run the mesh updater
pub fn process(dir: &Path) -> std::result::Result<(), std::io::Error> {
    let meshes = scan_dir_for_meshes(dir).unwrap();
    let mesh_bar = create_progress_bar(meshes.len() as u64);

    mesh_bar.set_prefix("Mesh Update");
    for mesh in meshes {
        mesh_bar.inc(1);
        mesh_bar.set_message(&format!("Updating {:?}...", &mesh));
        rename_image_references(&mesh)?;
    }

    // TODO: Update image references in material, txt, and sdf

    mesh_bar.finish_with_message("Meshes webified!");

    Ok(())
}
