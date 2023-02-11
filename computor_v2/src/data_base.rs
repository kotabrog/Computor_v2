use std::collections::HashMap;

use crate::num::Num;


#[derive(Debug, PartialEq)]
pub struct DataBase {
    data: HashMap<String, Num>,
}


impl DataBase {
    pub fn new() -> DataBase {
        DataBase {data: HashMap::new()}
    }

    pub fn register(&mut self, name: &String, num: Num) {
        let name = name.as_str().to_lowercase();
        self.data.insert(name, num);
    }

    pub fn get(&self, name: &String) -> Option<&Num> {
        self.data.get(name)
    }
}
