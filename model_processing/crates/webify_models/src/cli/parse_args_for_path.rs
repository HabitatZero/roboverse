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

#[cfg(test)]
mod parse_args_for_path_tests {
  use super::*;

  #[test]
  fn it_returns_the_correct_arg() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("foo/bar")); // Test base path
    args.push(String::from("/")); // Test provided arg path

    // Return the first argument as a path
    let arg = parse_args_for_path(&args).unwrap();
    assert_eq!(arg, Path::new("/"));
  }

  #[test]
  fn it_errors_on_no_path() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("foo/bar")); // Test base path

    let arg = parse_args_for_path(&args);
    assert!(arg.is_err());
  }

  #[test]
  fn it_errors_when_path_is_file() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("foo/bar")); // Test base path
    args.push(String::from("tests/README.md"));

    let arg = parse_args_for_path(&args);
    assert!(arg.is_err());
  }
}
