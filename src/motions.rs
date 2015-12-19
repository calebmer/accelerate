pub struct Motion {
    pub name: String,
    pub add_name: String,
    pub sub_name: String,
    pub add: String,
    pub sub: String,
}

impl Motion {
  pub fn get_test(add: String, sub: String) -> Self {
    Motion {
      name: "motion".to_string(),
      add_name: "motion.add".to_string(),
      sub_name: "motion.sub".to_string(),
      add: add,
      sub: sub,
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
  fn get(names: &Vec<&str>) {
    let template = Regex::new("/^([x.]+)([\\-_ ~]+)template\\.(add|sub)(.*)$/i").unwrap();
    let names_iter = names.iter();
  }
}

    }
}

fn read_dir(directory: &String) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(directory.to_string()) {
        for entry in entries {
            if let Ok(file_name) = entry.unwrap().file_name().into_string() {
                names.push(file_name);
            } else {
                panic!("File Name did not contain valid Unicode data");
            }
        }
    } else {
        panic!("Directory: '{}' not found!", directory);
    }
    names
}

/*
 pub fn discover(directory: &String) -> Vec<Motion>{
  let names = read_dir(directory);
  let template = Template::get(&names);

 }
*/

// TODO implement
pub fn get(directory: &String) -> Vec<Motion> {
  let motions: Vec<Motion> = Vec::new();
  if let Ok(files) = fs::read_dir(directory.to_string()) {
    for file in files {
      let path = file.unwrap().file_name();
      println!("Location: {}", path.to_str().unwrap());
    }
  } else {
    panic!("Directory: '{}' not found!", directory);
  }
  // TODO Replace this with proper result
  return vec![Motion::get_test("add 1".to_string(), "sub 1".to_string())];
}
