//! After renaming the files, we also need to update the references inside the meshes
//! to make sure that they are pointing at the right spot.

mod process;
mod rename_image_references;
mod scan_dir_for_meshes;

pub use self::process::process;
pub use self::rename_image_references::rename_image_references;
pub use self::scan_dir_for_meshes::scan_dir_for_meshes;
