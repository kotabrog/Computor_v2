use std::collections::HashMap;

use crate::num::Num;
use crate::binary_tree::BinaryTree;
use crate::parser::{Parser, Element};


#[derive(Debug, PartialEq)]
pub enum Data {
    Num(Num),
    Func(Box<(BinaryTree<Element>, String)>)
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

    pub fn register_func(&mut self, name: &String, tree: BinaryTree<Element>, variable: String) {
        let name = name.as_str().to_lowercase();
        self.data.insert(name, Data::Func(Box::new((tree, variable))));
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
            Data::Func(_) => None,
        }
    }

    pub fn get_func(&self, name: &String) -> Option<&Box<(BinaryTree<Element>, String)>> {
        let data = match self.get(name) {
            Some(d) => d,
            None => return None,
        };
        match data {
            Data::Num(_) => None,
            Data::Func(f) => Some(f),
        }
    }

    pub fn show_variable(&self) -> String {
        let mut string = String::new();
        for (key, value) in self.data.iter() {
            match value {
                Data::Num(n) => string += format!("{}: {}\n", key, n).as_str(),
                Data::Func(_) => {},
            }
        }
        if string.is_empty() {
            format!("No variables defined yet")
        } else {
            string
        }
    }

    pub fn show_function(&self) -> Result<String, String> {
        let mut string = String::new();
        for (key, value) in self.data.iter() {
            match value {
                Data::Num(_) => {},
                Data::Func(b) => {
                    let func_tree = &b.0;
                    let variable = &b.1;
                    let tree_string = Parser::print_tree(func_tree)?;
                    string += format!("{}({}): {}\n", key, variable, tree_string).as_str()
                },
            }
        }
        if string.is_empty() {
            Ok(format!("No functions defined yet"))
        } else {
            Ok(string)
        }
    }
}
