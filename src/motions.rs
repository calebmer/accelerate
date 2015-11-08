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

use std::fs;
// TODO implement
pub fn get(directory: String) -> Vec<Motion> {
    let motions: Vec<Motion> = Vec::new();
    let files = if let Ok(fs) = fs::read_dir(directory.clone()) {
        fs
    } else {
        panic!("Directory: '{}' not found!", directory);
    };

    for file in files {
        let path = file.unwrap().path();
        println!("Location: {}", path.to_str().unwrap());
    }

    // TODO Replace this with proper result
    return vec![Motion::get_test("add 1".to_string(), "sub 1".to_string())];
}

extern crate regex;
use self::regex::Regex;

// TODO implement
pub fn get_template(directory: String) {
    let template_add = Regex::new("/^([x.]+)([\\-_ ~]+)template\\.add(.*)$/i").unwrap();
    let template_sub = Regex::new("/^([x.]+)([\\-_ ~]+)template\\.sub(.*)$/i").unwrap();

}