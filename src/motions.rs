pub struct Motion {
  pub name: String,
  pub add_name: String,
  pub sub_name: String,
  pub add: String,
  pub sub: String,
}

impl Motion {
  fn get_test() -> Self {
    Motion {
      name: "motion".to_string(),
      add_name: "motion.add".to_string(),
      sub_name: "motion.sub".to_string(),
      add: "add".to_string(),
      sub: "sub".to_string(),
    }
  }
}

extern crate regex;
use self::regex::Regex;
use std::fs;

struct Template {
  add_name: String,
  sub_name: String,
  version: Vec<isize>,
  separator: String,
  extension: String,
  add: String,
  sub: String,
  regex: Regex,
}

// TODO implement
impl Template {
  fn get(names: &Vec<String>) {
    let template_add = Regex::new("/^([x.]+)([\\-_ ~]+)template\\.add(.*)$/i").unwrap();
    let template_sub = Regex::new("/^([x.]+)([\\-_ ~]+)template\\.sub(.*)$/i").unwrap();
    let mut names_iter = names.iter();
    let add_name = names_iter.find(|name| template_add.is_match(name)).unwrap();
    let sub_name = names_iter.find(|name| template_sub.is_match(name)).unwrap();

    if template_add.replace_all(add_name, "$1,$2,$3") != template_sub.replace_all(sub_name, "$1,$2,$3") {
      panic!("And and sub template names do not equal each other")
    }

    let captures = template_add.captures(add_name).unwrap();
    let version = captures.at(0).unwrap().split('.').map(|s| s.len());
    let seperator = captures.at(1).unwrap();
    let extension = captures.at(2).unwrap();

    // x.xx.xxx => [1, 2, 3] => ["\\d{1}", "\\d{2}", "\\d{3}"] => "\\.\\d{1}\\.\\d{2}\\.\\d{3}"
    let version_regex = version.map(|len| "\\d{".to_string() + &len.to_string() + "}").fold("".to_string(), |acc, re| (acc + "\\." + &re));
    let template_regex = Regex::new(&("/^(".to_string() + &version_regex + ")"));
  }
}

fn read_dir(directory: &str) -> Vec<String> {
  let mut names = Vec::new();
  if let Ok(entries) = fs::read_dir(directory.to_string()) {
    for entry in entries {
      if let Ok(file_name) = entry.unwrap().file_name().into_string() {
        names.push(file_name);
      } else {
        panic!("File name did not contain valid Unicode data");
      }
    }
  } else {
    panic!("Directory: '{}' not found!", directory);
  }
  names
}

// pub fn discover(directory: &String) -> Vec<Motion>{
// let names = read_dir(directory);
// let template = Template::get(&names);
// }
//

// TODO remove this in favor of real implementation
pub fn get() -> Vec<Motion> { return vec![Motion::get_test()]; }
