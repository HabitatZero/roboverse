//! Extract the path from the arguments provided, or return errors when there is none
//! or if a file is provided

use std::{
  io::{Error, ErrorKind},
  path::Path,
};

pub fn parse_args_for_path<'a>(
  args: &'a Vec<String>,
) -> std::result::Result<&'a std::path::Path, std::io::Error> {
  if args.len() <= 1 {
    return Err(Error::new(
      ErrorKind::Other,
      "Path not provided, no work to do.",
    ));
  }

  let path = Path::new(&args[1]);
  if !path.is_dir() {
    return Err(Error::new(
      ErrorKind::Other,
      "Path provided is a file, please provide a directory.",
    ));
  }

  Ok(path)
}
