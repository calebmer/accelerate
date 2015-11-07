pub struct Motion {
    pub name: String,
    pub add_name: String,
    pub sub_name: String,
    pub add: String,
    pub sub: String,
}

impl Motion {
    pub fn get(add: String, sub: String) -> Self {
        Motion {
            name: "motion".to_string(),
            add_name: "motion.add".to_string(),
            sub_name: "motion.sub".to_string(),
            add: add,
            sub: sub,
        }
    }
}

use std::fs;
// TODO implement
pub fn get(directory: String) -> Vec<Motion> {
  let paths = fs::read_dir(directory.clone());
  match paths {
    Ok(paths) => {
      for path in paths {
          // TODO add Motion::get that takes the file
          //println!("Name: {}", path);
      }
    },
    Err(e) => {
      panic!("Directory {0} did not exist!\n  Error: {1}", directory, e);
    }
  }
  // TODO Replace this
  return vec![Motion::get("add 1".to_string(), "sub 1".to_string())];
}
