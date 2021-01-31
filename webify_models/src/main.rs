use std::{fs, path::PathBuf, process::exit};
use std::env;
use std::path::Path;
use image::{io::Reader as ImageReader};
use image::ImageFormat::{Tiff};

use indicatif::{ProgressBar, ProgressStyle};
use console::style;

const TEXTURE_IMAGE_TYPES: [&str; 7] = [
    r#"tif"#,
    r#"tga"#,
    r#"tiff"#,
    r#"jpeg"#,
    r#"jpg"#,
    r#"gif"#,
    r#"png"#
];

#[derive(Debug, Clone)]
struct Image {
    path: PathBuf,
    extension: String,
}

fn scan_dir_for_images(dir: &Path, mut images: Vec<Image>) -> std::io::Result<Vec<Image>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let e = entry?;
            let path = e.path();

            if path.is_dir() {
                images = scan_dir_for_images(&path, images.clone())?;
            } else {
                let extension = match path.extension() {
                    Some(ext) => ext.to_str().unwrap(),
                    _ => ""
                };

                if TEXTURE_IMAGE_TYPES.contains(&extension) {
                    images.push(Image { path: path.clone(), extension: extension.to_string() });
                };
            }
        }
    }
    Ok(images)
}

fn convert_to_png(mut image: Image, progress_bar: &ProgressBar) -> std::io::Result<Image> {
    progress_bar.set_prefix("PNG Conversion");
    let styled_path = style(image.path.to_string_lossy()).dim();
    if image.extension == "png" {
        progress_bar.set_message(&format!("{} already in PNG, skipping", styled_path));
        return Ok(image);
    }

    progress_bar.set_message(&format!("Converting {}...", styled_path));
    let image_reader = match ImageReader::open(image.path.clone()) {
        Ok(img) => img,
        Err(e) => panic!("Failed to open image during PNG conversion: {:?}", e),
    };

    // Somehow, Tiff conversion is problematic, so we'll skip that
    if image_reader.format().is_some() && image_reader.format() != Some(Tiff) {
        let img = match image_reader.decode() {
            Ok(i) => i,
            Err(e) => panic!("Failed to open image during PNG conversion: {:?}", e),
        };

        match img.save(image.path.with_extension("png")) {
            Ok(_) => "",
            Err(e) => panic!("Could not convert {:?} to PNG: {:?}", image.path, e),
        };

        fs::remove_file(&image.path)?;
        progress_bar.set_message(&format!("{} converted!", styled_path));
        image.path = image.path.with_extension("png");
    } else {
        progress_bar.set_message(&format!("{} skipped, as conversion of this file is problematic at this moment.", styled_path));
    }

    Ok(image)
}

fn move_to_textures_dir<'a>(mut image: Image, base_path: &Path, progress_bar: &ProgressBar) -> std::io::Result<Image> {
    let file_name = image.path.file_name().unwrap().to_str().unwrap();
    let textures_path = Path::new("materials").join("textures").join(file_name);
    let meshes_path = Path::new("meshes").join(file_name);

    progress_bar.set_prefix("Texture Move");
    let styled_path = style(image.path.to_string_lossy()).dim();

    if !image.path.ends_with(textures_path) && !image.path.ends_with(meshes_path) {
        progress_bar.set_message(&format!("Moving {} to textures directory...", styled_path));
        let mut model_path_ancestors = image.path.strip_prefix(base_path).unwrap().ancestors();
        let model_path = model_path_ancestors.nth(model_path_ancestors.count() - 2).unwrap(); // There must be a better way to do this...
        let new_textures_path = base_path.join(model_path).join("materials").join("textures");

        fs::create_dir_all(&new_textures_path)?;
        progress_bar.set_message(&format!("Created {}", style(&new_textures_path.to_string_lossy()).dim()));
        fs::copy(&image.path, new_textures_path.join(file_name))?;
        fs::remove_file(&image.path)?;
        let new_textures_path_with_ext = new_textures_path.join(file_name).clone();
        progress_bar.set_message(&format!("Moved {} to {}", style(styled_path).dim(), style(new_textures_path_with_ext.to_string_lossy()).dim()));
        image.path = new_textures_path_with_ext;
    }

    Ok(image)
}

fn main() -> std::result::Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    println!("{}", style("BelvedereSim").underlined().bold().white());
    if args.len() <= 1 {
        println!("Path not provided, no work to do.");
        exit(1)
    }

    let path = Path::new(&args[1]);
    if !path.is_dir() {
        println!("Path provided is a file, please provide a directory.");
        exit(1)
    }

    println!("\nScanning for images to webify...");
    println!("{}", &style("Note that webify models is a destructive action and will DELETE the existing non-PNG files.").on_red());
    let mut images = match scan_dir_for_images(path, Vec::new()) {
        Err(error) => panic!("Failed to scan all directories for images: {:?}", error),
        Ok(image_list) => image_list
    };
    images.sort_by(|a, b| b.extension.cmp(&a.extension));

    println!("Images found: {}\n", style(images.len()).bold().blue());



    let bar = ProgressBar::new(images.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{prefix:15.green.bold} {wide_msg}\n[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7}")
        .progress_chars("█▓▒░")
    );

    for image in images {
        bar.inc(1);
        let moved_image = move_to_textures_dir(image, path, &bar).unwrap();
        convert_to_png(moved_image, &bar).unwrap();
    }
    bar.finish_with_message("Models webified!");

    // Update texture path in DAE files

    Ok(())
}
