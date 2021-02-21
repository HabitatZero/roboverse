//! Rename all image references inside a DAE mesh to be .PNG and point
//! at the right textures path

use std::path::PathBuf;
use std::{
    fs::{self},
    path::Path,
};

use aho_corasick::AhoCorasickBuilder;

/// Orchestrator to rename image references in a DAE mesh
pub fn rename_image_references(mesh: &PathBuf) -> std::result::Result<(), std::io::Error> {
    let result = find_and_rename_image_references(mesh)?;
    let final_result = update_texture_path(result)?;
    fs::write(mesh, final_result)?;

    Ok(())
}

/// Rename all occurences of supported image types to PNG
fn find_and_rename_image_references(mesh: &PathBuf) -> std::result::Result<String, std::io::Error> {
    let patterns = &[".jpg", "_jpg", ".tga", "_tga"];
    let f = fs::read_to_string(mesh)?;

    let ac = AhoCorasickBuilder::new().build(patterns);
    let result = ac.replace_all(&f, &[".png", "_png", ".png", "_png"]);

    Ok(result)
}

/// Update all texture paths to point to the relative path for textures/materials
fn update_texture_path(contents: String) -> std::result::Result<String, std::io::Error> {
    let texture_element = &["<init_from>", "</init_from>"];
    let ac = AhoCorasickBuilder::new().build(texture_element);
    let texture_path = Path::new("materials").join("textures");

    let mut new_contents = String::new();

    // Find all texture elements
    for line in contents.lines() {
        let mut start = 0;
        let mut new_line: String = String::new();
        for mat in ac.find_iter(&line) {
            if mat.pattern() == 0 {
                start = mat.end();
            } else if mat.pattern() == 1 {
                let texture_name = &line[start..mat.start()];
                // Prefix image reference with relative directory path to textures
                if texture_name.ends_with(".png")
                    && !texture_name.contains(&texture_path.to_str().unwrap())
                {
                    // TODO: Properly find the root path of the mesh, rather than assuming
                    new_line = line.replace(
                        texture_name,
                        Path::new("..")
                            .join("materials")
                            .join("textures")
                            .join(texture_name)
                            .to_str()
                            .unwrap(),
                    );
                }
            }
        }
        // TODO: Fix the output of this to keep the newlines
        if new_line.is_empty() {
            new_contents.push_str(format!("{}\n", line).as_str());
        } else {
            new_contents.push_str(format!("{}\n", new_line).as_str());
        }
    }

    Ok(new_contents)
}

#[cfg(test)]
mod rename_image_references_tests {
    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    fn setup(test_run_id: &str) -> std::result::Result<(), std::io::Error> {
        let example_path = Path::new("tests").join("mesh_update").join("test");
        let destination_path = Path::new("tests").join("mesh_update").join(test_run_id);

        fs::create_dir_all(&destination_path.join("meshes"))?;
        fs::create_dir_all(&destination_path.join("materials").join("textures"))?;

        fs::copy(
            example_path.join("meshes").join("test.dae"),
            &destination_path.join("meshes").join("test.dae"),
        )?;

        Ok(())
    }

    fn teardown(test_run_id: &str) -> std::result::Result<(), std::io::Error> {
        let destination_path = Path::new("tests").join("mesh_update").join(test_run_id);
        fs::remove_dir_all(destination_path)?;

        Ok(())
    }

    #[test]
    fn it_renamed_and_updated_texture_references() -> std::result::Result<(), std::io::Error> {
        let test_run_id = "rename_and_update_texture_1";

        setup(&test_run_id)?;

        let destination_path = Path::new("tests")
            .join("mesh_update")
            .join(&test_run_id)
            .join("meshes")
            .join("test.dae");
        rename_image_references(&destination_path)?;

        let mut file = File::open(destination_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        assert_eq!(contents, "<!-- This is not a valid DAE, just a test file -->\n\n<image id=\"Test_Diffuse_png\">\n  <init_from>../materials/textures/test_diffuse.png</init_from>\n</image>\n");

        teardown(&test_run_id)?;

        Ok(())
    }
}

#[cfg(test)]
mod find_and_rename_image_references_tests {
    use super::*;

    #[test]
    fn it_renamed_texture_references() -> std::result::Result<(), std::io::Error> {
        let destination_path = Path::new("tests")
            .join("mesh_update")
            .join("test")
            .join("meshes")
            .join("test.dae");
        let result = find_and_rename_image_references(&destination_path)?;
        assert_eq!(result, "<!-- This is not a valid DAE, just a test file -->\n\n<image id=\"Test_Diffuse_png\">\n  <init_from>test_diffuse.png</init_from>\n</image>\n");

        Ok(())
    }
}

#[cfg(test)]
mod update_texture_path_tests {
    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn it_updated_texture_paths() -> std::result::Result<(), std::io::Error> {
        let destination_path = Path::new("tests")
            .join("mesh_update")
            .join("already_png")
            .join("meshes")
            .join("already_png.dae");

        let mut file = File::open(destination_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let result = update_texture_path(contents)?;
        assert_eq!(result, "<!-- This is not a valid DAE, just a test file -->\n\n<image id=\"Test_Diffuse_png\">\n  <init_from>../materials/textures/test_diffuse.png</init_from>\n</image>\n");

        Ok(())
    }
}
