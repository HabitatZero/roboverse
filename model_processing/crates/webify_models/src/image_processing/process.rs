//! Orchestrator to convert texture images from whatever format they're in to PNG

use std::path::Path;

use console::style;

use crate::cli::create_progress_bar;
use crate::image_processing::{convert_to_png, move_to_textures_dir, scan_dir_for_images};

/// Orchestrator to convert texture images from whatever format they're in to PNG
pub fn process(dir: &Path) -> std::result::Result<(), std::io::Error> {
    let images = scan_dir_for_images(dir).unwrap();
    let image_bar = create_progress_bar(images.len() as u64);

    image_bar.set_prefix("Texture Move");
    for image in images {
        image_bar.inc(1);
        let styled_path = style(image.path.to_string_lossy()).dim().to_string();

        image_bar.set_message(&format!("Moving {} to textures directory...", styled_path));
        let moved_image = move_to_textures_dir(image, dir)?;
        let moved_image_path = style(moved_image.path.to_string_lossy()).dim().to_string();
        image_bar.set_message(&format!("Moved {} to {}", styled_path, moved_image_path));

        image_bar.set_prefix("PNG Conversion");
        if moved_image.extension == "png" {
            image_bar.set_message(&format!("{} already in PNG, skipping", moved_image_path));
        } else {
            image_bar.set_message(&format!("Converting {}...", moved_image_path));
            convert_to_png(moved_image)?;
            image_bar.set_message(&format!("{} converted!", moved_image_path));
        }
    }
    image_bar.finish_with_message("Images webified!");

    Ok(())
}
