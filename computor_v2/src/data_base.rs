use std::collections::HashMap;

use crate::num::Num;
use crate::binary_tree::BinaryTree;
use crate::parser::{Parser, Element};
use crate::functions;


#[derive(Debug, PartialEq)]
pub enum Data {
    Num(Num),
    Func(Box<(BinaryTree<Element>, String)>),
}


#[derive(Debug, PartialEq)]
pub struct DataBase {
    data: HashMap<String, Data>,
    built_in: HashMap<String, Data>,
}


impl DataBase {
    pub fn new() -> DataBase {
        let mut built_in = HashMap::new();
        Self::built_in_insert(&mut built_in, "exp".to_string());
        Self::built_in_insert(&mut built_in, "sqrt".to_string());
        DataBase { data: HashMap::new(), built_in }
    }

    fn built_in_insert(built_in: &mut HashMap<String, Data>, func_name: String) {
        built_in.insert(func_name.clone(), Data::Func(functions::make_builtin_func_box(func_name)));
    }

    pub fn register_num(&mut self, name: &String, num: Num) -> Result<(), String> {
        let name = name.as_str().to_lowercase();
        match self.built_in.get(&name) {
            None => {},
            Some(_) => return Err("The variable cannot be registered".to_string())
        }
        self.data.insert(name, Data::Num(num));
        Ok(())
    }

    pub fn register_func(&mut self, name: &String, tree: BinaryTree<Element>, variable: String) -> Result<(), String> {
        let name = name.as_str().to_lowercase();
        match self.built_in.get(&name) {
            None => {},
            Some(_) => return Err("The function cannot be registered".to_string())
        }
        self.data.insert(name, Data::Func(Box::new((tree, variable))));
        Ok(())
    }

    pub fn get(&self, name: &String) -> Option<&Data> {
        let name = name.as_str().to_lowercase();
        match self.built_in.get(&name) {
            Some(data) => Some(&data),
            None => self.data.get(&name),
        }
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

    pub fn get_builtin_func(&self, name: &String) -> Option<&Box<(BinaryTree<Element>, String)>> {
        let data = match self.built_in.get(name) {
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
            format!("No variables defined yet\n")
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
            Ok(format!("No functions defined yet\n"))
        } else {
            Ok(string)
        }
    }
}
