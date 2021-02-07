//! Webify_models is a script to convert Gazebo models to a format that
//! that is friendlier for the web, mainly, to use PNGs and to move the
//! texture files in consistent paths. To be honest, I'm not entirely sure
//! this is necessary as browsers render pretty much everything nowadays,
//! but hey, I'm not going to shake the tree too much before I fully understand
//! the purpose of all these things are in somebody else's project that I'm rewriting.

use std::env;
use std::process::exit;

use console::style;

mod cli;
mod image_processing;
mod mesh_update;

fn main() -> std::result::Result<(), std::io::Error> {
  println!("{}", style("Roboverse").underlined().bold().white());

  let args: Vec<String> = env::args().collect();
  let path = match cli::parse_args_for_path(&args) {
    Ok(p) => p,
    Err(e) => {
      println!("{}", e);
      exit(1)
    }
  };

  image_processing::process(path)?;
  mesh_update::process(path)?;

  Ok(())
}
