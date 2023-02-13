use std::collections::HashMap;

use crate::num::Num;
use crate::binary_tree::BinaryTree;
use crate::parser::Element;


#[derive(Debug, PartialEq)]
pub enum Data {
    Num(Num),
    Func(BinaryTree<Element>)
}


#[derive(Debug, PartialEq)]
pub struct DataBase {
    data: HashMap<String, Data>,
}


impl DataBase {
    pub fn new() -> DataBase {
        DataBase {data: HashMap::new()}
    }

    pub fn register_num(&mut self, name: &String, num: Num) {
        let name = name.as_str().to_lowercase();
        self.data.insert(name, Data::Num(num));
    }

    pub fn get(&self, name: &String) -> Option<&Data> {
        let name = name.as_str().to_lowercase();
        self.data.get(&name)
    }

    pub fn get_num(&self, name: &String) -> Option<&Num> {
        let data = match self.get(name) {
            Some(d) => d,
            None => return None,
        };
        match data {
            Data::Num(n) => Some(n),
            Data::Func(f) => None,
        }
    }
}
