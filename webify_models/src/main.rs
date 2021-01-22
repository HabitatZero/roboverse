use std::{fs, path::PathBuf, process::exit};
use std::env;
use std::path::Path;

const TEXTURE_IMAGE_TYPES: [&str; 7] = [
    r#"tif"#,
    r#"tga"#,
    r#"tiff"#,
    r#"jpeg"#,
    r#"jpg"#,
    r#"gif"#,
    r#"png"#
];

fn scan_dir_for_images(dir: &Path, mut images: Vec<PathBuf>) -> std::io::Result<Vec<PathBuf>> {
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
                    images.push(path);
                };
            }
        }
    }
    Ok(images)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Path not provided, no work to do.");
        exit(1)
    }

    let path = Path::new(&args[1]);

    if path.is_dir() {
        let images = match scan_dir_for_images(path, Vec::new()) {
            Err(error) => panic!("Failed to scan all directories for images: {:?}", error),
            Ok(image_list) => image_list
        };
        println!("{:?}", images);
        println!("Images found: {:?}", images.len());
    }
}
