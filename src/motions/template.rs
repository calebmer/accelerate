extern crate regex;
use self::regex::Regex;
use std::fs;

pub enum Op {
  Add,
  Sub,
}

pub struct Template {
  pub add_name: String,
  pub sub_name: String,
  pub version: Vec<usize>,
  pub separator: String,
  pub extension: String,
  pub add: String,
  pub sub: String,
  pub regex: Regex,
}

impl Template {
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

  pub fn get_name(&self, name: &String) -> String {
    if !self.regex.is_match(name) {
      panic!("Name must conform with the template string");
    }

    self.regex.captures(name).unwrap().at(2).unwrap().to_string()
  }
}
