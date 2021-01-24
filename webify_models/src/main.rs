use std::{fs, path::PathBuf, process::exit};
use std::env;
use std::path::Path;
use image::io::Reader as ImageReader;
use image::ImageFormat::Jpeg;

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

fn convert_to_png(mut image: Image) -> std::io::Result<Image> {
    if image.extension == "png" {
        println!("Skipping conversion for {:?}", image.path);
        return Ok(image);
    }

    println!("Converting {:?}...", image.path);
    let image_reader = match ImageReader::open(image.path.clone()) {
        Ok(img) => img,
        Err(e) => panic!("Failed to open image during PNG conversion: {:?}", e),
    };

    if image_reader.format() == Some(Jpeg) {
        let img = match image_reader.decode() {
            Ok(i) => i,
            Err(e) => panic!("Failed to open image during PNG conversion: {:?}", e),
        };
        match img.save(image.path.with_extension("png")) {
            Ok(_) => println!("Converted {:?} to PNG!", image.path),
            Err(e) => panic!("Could not convert {:?} to PNG: {:?}", image.path, e),
        };

        fs::remove_file(&image.path)?;
        println!("Deleted {:?}", &image.path);
        image.path = image.path.with_extension("png");
    }

    Ok(image)
}

fn move_to_textures_dir<'a>(mut image: Image, base_path: &Path) -> std::io::Result<Image> {
    let file_name = image.path.file_name().unwrap().to_str().unwrap();
    let textures_path = Path::new("materials").join("textures").join(file_name);
    let meshes_path = Path::new("meshes").join(file_name);

    if !image.path.ends_with(textures_path) && !image.path.ends_with(meshes_path) {
        println!("{:?}", image.path);
        let mut model_path_ancestors = image.path.strip_prefix(base_path).unwrap().ancestors();
        let model_path = model_path_ancestors.nth(model_path_ancestors.count() - 2).unwrap(); // There must be a better way to do this...
        let new_textures_path = base_path.join(model_path).join("materials").join("textures");

        fs::create_dir_all(&new_textures_path)?;
        fs::copy(&image.path, new_textures_path.join(file_name))?;
        fs::remove_file(&image.path)?;
        image.path = new_textures_path.join(file_name).clone();
    }

    Ok(image)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Path not provided, no work to do.");
        exit(1)
    }

    let path = Path::new(&args[1]);
    if !path.is_dir() {
        println!("Path provided is a file, please provide a directory.");
        exit(1)
    }

    let mut images = match scan_dir_for_images(path, Vec::new()) {
        Err(error) => panic!("Failed to scan all directories for images: {:?}", error),
        Ok(image_list) => image_list
    };
    images.sort_by(|a, b| b.extension.cmp(&a.extension));
    println!("Images found: {:?}", images.len());

    for image in images {
        let moved_image = move_to_textures_dir(image, path).unwrap();
        convert_to_png(moved_image).unwrap();
    }

    // Update texture path in DAE files
}
