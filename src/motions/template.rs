//! Motions module helper for getting the template information in the file
//! system‘s motions directory.
extern crate regex;
use self::regex::Regex;
use std::fs;

/// Simple enum defining the add and sub operations.
pub enum Op {
  Add,
  Sub,
}

/// The template type which contains a number of useful information when
/// discovering and creating motions.
pub struct Template {
  /// The name of the add template file.
  pub add_name: String,
  /// The name of the sub template file.
  pub sub_name: String,
  /// An array defining the structure of the template‘s semantic version. If a
  /// template defines a version of `xxx` the resulting vector will be roughly
  /// `[3]`. But if a template defines a version of `x.x.xx` the resulting
  /// vector will be roughly `[1,1,2]`.
  pub version: Vec<usize>,
  /// The string between the version and the template name.
  pub separator: String,
  /// The optional extension of the template.
  pub extension: String,
  /// The contents of the add template file.
  pub add: String,
  /// The contents of the sub template file.
  pub sub: String,
  /// A constructed regular expression which will match all motions which the
  /// template matches.
  pub regex: Regex,
}

impl Template {
  /// Gets the template for a motions directory. Searches all the file names
  /// for a pair of files matching the accelerator template format and turns
  /// those into a usable type by the motions module.
  pub fn get(directory: &String, names: &Vec<String>) -> Self {
    let template_add = Regex::new(r"^([x.]+)([-_ ~]+)template\.add(.*)$").unwrap();
    let template_sub = Regex::new(r"^([x.]+)([-_ ~]+)template\.sub(.*)$").unwrap();
    let add_name = names.iter().find(|a| template_add.is_match(a)).unwrap();
    let sub_name = names.iter().find(|a| template_sub.is_match(a)).unwrap();

    if template_add.replace_all(add_name, "$1,$2,$3") != template_sub.replace_all(sub_name, "$1,$2,$3") {
      panic!("And and sub template names do not equal each other")
    }

    let captures = template_add.captures(add_name).unwrap();
    let version: Vec<usize> = captures.at(1).unwrap().split('.').map(|s| s.len()).collect();
    let separator = captures.at(2).unwrap();
    let extension = captures.at(3).unwrap();

    let mut version_regex = version.iter().map(|len| r"\d{".to_string() + &len.to_string() + "}").fold("".to_string(), |acc, re| (acc + &re + r"\."));
    let len = version_regex.len();
    version_regex.pop();
    version_regex.pop();
    let regx_str = &(String::from("^(") + &version_regex + ")" + &regex::quote(separator) + r"(.+)\.(add|sub)" + &regex::quote(extension) + "$");
    Template {
      add_name: add_name.clone(),
      sub_name: sub_name.clone(),
      version: captures.at(1).unwrap().split('.').map(|s| s.len()).collect(),
      separator: separator.to_string(),
      extension: extension.to_string(),
      add: super::read_file(&directory, add_name),
      sub: super::read_file(&directory, sub_name),
      regex: Regex::new(regx_str).unwrap(),
    }
  }

  /// Get’s the operation (add or sub) from a motion‘s name string. For
  /// example, a motion that matches the template and looks like
  /// `001-hello-world.add` would return as an “add” operation.
  pub fn get_op(&self, name: &String) -> Op {
    if !self.regex.is_match(name) {
      panic!("Name must conform with the template string")
    }

    let op_string = self.regex.captures(name).unwrap().at(3).unwrap();

    match op_string {
      "add" => Op::Add,
      "sub" => Op::Sub,
      _ => unreachable!(),
    }
  }

  /// Get‘s the name of a motion without the version number, seperator,
  /// operation, or extension. For example, a motion that matches the template
  /// and looks like `001-hello-world.add` would return `hello-world`.
  pub fn get_name(&self, name: &String) -> String {
    if !self.regex.is_match(name) {
      panic!("Name must conform with the template string");
    }

    self.regex.captures(name).unwrap().at(2).unwrap().to_string()
  }
}
