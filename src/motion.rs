use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use regex;
use regex::Regex;
use error::Error;

#[derive(Eq, PartialEq, Debug)]
pub struct Motion {
  pub name: String,
  pub add_path: PathBuf,
  pub sub_path: PathBuf,
}

#[derive(Eq, PartialEq, Debug)]
struct Template {
  extension: String,
  add_path: PathBuf,
  sub_path: PathBuf,
}

fn find(dir: &Path) -> Result<Vec<Motion>, Error> {
  let template = try!(find_template(dir));
  let motions = try!(find_motions(&template, dir));
  Ok(motions)
}

fn find_template(dir: &Path) -> Result<Template, Error> {
  let paths = try!(find_paths(dir.to_path_buf(), 1));

  // Create the regexi which will match our template file names.
  let add_re = Regex::new(r"^template\.add($|\..+$)").unwrap();
  let sub_re = Regex::new(r"^template\.sub($|\..+$)").unwrap();

  // Find our add and sub paths in our list of paths. Error if we can’t find
  // anything.
  let add_path = try!(
    paths.iter()
    .find(|path| path.file_name().and_then(OsStr::to_str).map(|s| add_re.is_match(s)).unwrap_or(false))
    .ok_or(Error::new("Add template file was not found."))
  );

  let sub_path = try!(
    paths.iter()
    .find(|path| path.file_name().and_then(OsStr::to_str).map(|s| sub_re.is_match(s)).unwrap_or(false))
    .ok_or(Error::new("Sub template file was not found."))
  );

  // Get both the add extension and the sub extension. We can safely unwrap the
  // file names because part of being an `add_path` or `sub_path` means we
  // already checked that a file name exists.
  let add_ext = add_re.replace_all(add_path.file_name().unwrap().to_str().unwrap(), "$1");
  let sub_ext = sub_re.replace_all(sub_path.file_name().unwrap().to_str().unwrap(), "$1");

  // If the extensions are not equal, there is an error.
  if add_ext != sub_ext {
    return Err(Error::new(format!("Template extensions for add ('{}') and sub ('{}') do not match.", add_ext, sub_ext)))
  }

  // Return our template.
  Ok(Template {
    extension: add_ext,
    add_path: add_path.to_path_buf(),
    sub_path: sub_path.to_path_buf(),
  })
}

fn find_motions(template: &Template, dir: &Path) -> Result<Vec<Motion>, Error> {
  // Get all the file names for our paths for later use. We also make sure we
  // return a tuple. This way we can keep the original path.
  let paths = try!(find_paths(dir.to_path_buf(), 100));
  let paths: Vec<(&PathBuf, &str)> = {
    paths
    .iter()
    .filter_map(|path| path.file_name().and_then(OsStr::to_str).map(|file_name| (path, file_name)))
    .collect()
  };

  // Create the filename regexi for add and sub using the template extension.
  let add_re = Regex::new(&(r"^(\d{6}-.+)\.add".to_owned() + &regex::quote(&template.extension) + "$")).unwrap();
  let sub_re = Regex::new(&(r"^(\d{6}-.+)\.sub".to_owned() + &regex::quote(&template.extension) + "$")).unwrap();
  // Construct a motions accumulator.
  let mut motions: Vec<Motion> = Vec::new();

  // Iterate through all of our paths…
  for &(ref add_path, add_file_name) in paths.iter() {
    // If this path is an add file continue…
    if add_re.is_match(add_file_name) {
      // Get the name and timestamp for this motion.
      let name = add_re.replace_all(add_file_name, "$1");
      // Get the sub path with a name that matches our add path. If it does not
      // exist, throw an error.
      let &(ref sub_path, _) = try!(
        paths
        .iter()
        .find(|&&(_, sub_file_name)| sub_re.is_match(sub_file_name) && sub_re.replace_all(sub_file_name, "$1") == name)
        .ok_or(Error::new(format!("Sub file not found for add file '{}'.", add_path.display())))
      );
      // Add the motion to our accumulator.
      motions.push(Motion {
        name: name,
        add_path: add_path.to_path_buf(),
        sub_path: sub_path.to_path_buf(),
      });
    }
  }

  // Sort our motions by *file* name.
  motions.sort_by(|a, b| a.name.cmp(&b.name));

  // Return all of our motions.
  Ok(motions)
}

fn find_paths(path: PathBuf, recurse: u8) -> io::Result<Vec<PathBuf>> {
  // If the path is a directory let’s recursively go through every entry and
  // rerun our `discover_all` function.
  if try!(fs::metadata(&path)).is_dir() {
    // If we have ended our recursion, just return an empty vec.
    if recurse == 0 {
      return Ok(vec![])
    }
    // Create a new paths vec.
    let mut paths: Vec<PathBuf> = Vec::new();
    // Loop through the directory…
    for entry in try!(fs::read_dir(&path)) {
      // Get all the paths from this entry path by recursively calling the
      // function.
      let mut next_paths = try!(find_paths(try!(entry).path(), recurse - 1));
      // Append these next paths to our top level motions vec.
      paths.append(&mut next_paths);
    }
    // Return all of our motions.
    Ok(paths)
  }
  // Otherwise our path is not a directory and we should return a singleton
  // vector of the path.
  else {
    Ok(vec![path])
  }
}

#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};
  use super::{find_paths, find, Motion, find_template, Template};

  fn pb(path: &str) -> PathBuf {
    Path::new(path).to_path_buf()
  }

  #[test]
  fn test_find_paths() {
    assert_eq!(find_paths(pb("tests/fixtures/nested"), 100).unwrap(), vec![
      pb("tests/fixtures/nested/234567-bar.add"),
      pb("tests/fixtures/nested/234567-bar.sub"),
      pb("tests/fixtures/nested/a/345678-baz.add"),
      pb("tests/fixtures/nested/a/345678-baz.sub"),
      pb("tests/fixtures/nested/b/123456-foo.add"),
      pb("tests/fixtures/nested/b/123456-foo.sub"),
      pb("tests/fixtures/nested/b/c/456789-qux.add"),
      pb("tests/fixtures/nested/b/c/456789-qux.sub"),
      pb("tests/fixtures/nested/template.add"),
      pb("tests/fixtures/nested/template.sub"),
    ]);
  }

  #[test]
  fn test_find_paths_limit_recurse_1() {
    assert_eq!(find_paths(pb("tests/fixtures/nested"), 1).unwrap(), vec![
      pb("tests/fixtures/nested/234567-bar.add"),
      pb("tests/fixtures/nested/234567-bar.sub"),
      pb("tests/fixtures/nested/template.add"),
      pb("tests/fixtures/nested/template.sub"),
    ]);
  }

  #[test]
  fn test_find_paths_limit_recurse_2() {
    assert_eq!(find_paths(pb("tests/fixtures/nested"), 2).unwrap(), vec![
      pb("tests/fixtures/nested/234567-bar.add"),
      pb("tests/fixtures/nested/234567-bar.sub"),
      pb("tests/fixtures/nested/a/345678-baz.add"),
      pb("tests/fixtures/nested/a/345678-baz.sub"),
      pb("tests/fixtures/nested/b/123456-foo.add"),
      pb("tests/fixtures/nested/b/123456-foo.sub"),
      pb("tests/fixtures/nested/template.add"),
      pb("tests/fixtures/nested/template.sub"),
    ]);
  }

  #[test]
  fn test_fixtures_basic() {
    assert_eq!(find(Path::new("tests/fixtures/basic")).unwrap(), vec![
      Motion {
        name: "123456-foo".to_string(),
        add_path: pb("tests/fixtures/basic/123456-foo.add"),
        sub_path: pb("tests/fixtures/basic/123456-foo.sub"),
      },
      Motion {
        name: "234567-bar".to_string(),
        add_path: pb("tests/fixtures/basic/234567-bar.add"),
        sub_path: pb("tests/fixtures/basic/234567-bar.sub"),
      },
    ]);
  }

  #[test]
  fn test_template_basic() {
    assert_eq!(find_template(Path::new("tests/fixtures/basic")).unwrap(), Template {
      extension: "".to_string(),
      add_path: pb("tests/fixtures/basic/template.add"),
      sub_path: pb("tests/fixtures/basic/template.sub"),
    });
  }

  #[test]
  fn test_fixtures_nested() {
    assert_eq!(find(Path::new("tests/fixtures/nested")).unwrap(), vec![
      Motion {
        name: "123456-foo".to_string(),
        add_path: pb("tests/fixtures/nested/b/123456-foo.add"),
        sub_path: pb("tests/fixtures/nested/b/123456-foo.sub"),
      },
      Motion {
        name: "234567-bar".to_string(),
        add_path: pb("tests/fixtures/nested/234567-bar.add"),
        sub_path: pb("tests/fixtures/nested/234567-bar.sub"),
      },
      Motion {
        name: "345678-baz".to_string(),
        add_path: pb("tests/fixtures/nested/a/345678-baz.add"),
        sub_path: pb("tests/fixtures/nested/a/345678-baz.sub"),
      },
      Motion {
        name: "456789-qux".to_string(),
        add_path: pb("tests/fixtures/nested/b/c/456789-qux.add"),
        sub_path: pb("tests/fixtures/nested/b/c/456789-qux.sub"),
      },
    ]);
  }

  #[test]
  fn test_template_nested() {
    assert_eq!(find_template(Path::new("tests/fixtures/nested")).unwrap(), Template {
      extension: "".to_string(),
      add_path: pb("tests/fixtures/nested/template.add"),
      sub_path: pb("tests/fixtures/nested/template.sub"),
    });
  }

  #[test]
  fn test_fixtures_extension() {
    assert_eq!(find(Path::new("tests/fixtures/extension")).unwrap(), vec![
      Motion {
        name: "123456-foo".to_string(),
        add_path: pb("tests/fixtures/extension/123456-foo.add.sql"),
        sub_path: pb("tests/fixtures/extension/123456-foo.sub.sql"),
      },
      Motion {
        name: "234567-bar".to_string(),
        add_path: pb("tests/fixtures/extension/234567-bar.add.sql"),
        sub_path: pb("tests/fixtures/extension/234567-bar.sub.sql"),
      },
    ]);
  }

  #[test]
  fn test_template_extension() {
    assert_eq!(find_template(Path::new("tests/fixtures/extension")).unwrap(), Template {
      extension: ".sql".to_string(),
      add_path: pb("tests/fixtures/extension/template.add.sql"),
      sub_path: pb("tests/fixtures/extension/template.sub.sql"),
    });
  }

  #[test]
  fn test_fixtures_bad_templateless() {
    assert!(find(Path::new("tests/fixtures/bad/templateless")).is_err());
  }

  #[test]
  fn test_bad_names() {
    assert!(find(Path::new("tests/fixtures/bad/names")).is_err());
  }
}
