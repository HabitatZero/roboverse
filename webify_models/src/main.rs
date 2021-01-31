use std::process::exit;
use std::env;

use indicatif::{ProgressBar};
use console::style;

mod cli;
mod image_processing;
mod mesh_update;

fn main() -> std::result::Result<(), std::io::Error> {
    println!("{}", style("BelvedereSim").underlined().bold().white());

    let args: Vec<String> = env::args().collect();
    let path = match cli::parse_args_for_path(&args) {
        Ok(p) => p,
        Err(e) => { println!("{}", e); exit(1) }
    };

    image_processing::process(path)?;

    // Update texture path in DAE files
    println!("\nScanning for meshes to webify...");
    let mut meshes = match mesh_update::scan_dir_for_meshes(path, Vec::new()) {
        Err(error) => panic!("Failed to scan all directories for meshes: {:?}", error),
        Ok(mesh_list) => mesh_list
    };
    let mesh_bar  = cli::create_progress_bar(meshes.len() as u64);

    println!("Meshes found: {}\n", style(meshes.len()).bold().blue());
    for mesh in meshes {
        mesh_bar.inc(1);
        // update_mesh_paths(mesh, &bar).unwrap();
    }
    mesh_bar.finish_with_message("Meshes webified!");

    Ok(())
}
