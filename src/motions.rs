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

// TODO implement
pub fn get(directory: String) -> Vec<Motion> {
    return vec![Motion::get("add 1".to_string(), "sub 1".to_string())];
}
