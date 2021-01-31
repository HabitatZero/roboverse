use std::path::PathBuf;

pub fn rename_image_references(mesh: &PathBuf) -> std::result::Result<(), std::io::Error> {
  println!("{:?}", mesh);

  Ok(())
}
