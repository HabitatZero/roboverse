use std::process::exit;
use std::env;

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
    mesh_update::process(path)?;

    Ok(())
}
