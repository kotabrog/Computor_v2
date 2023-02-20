use crate::num::Num;
use crate::binary_tree::BinaryTree;
use crate::lexer::Token;
use crate::operator::Operator;
use crate::data_base::{DataBase, Data};
use crate::functions::builtin_func;


#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Dummy,
    Operator(Operator),
    Num(Num),
    Variable(Box<String>),
    Func(Box<String>),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn is_show_variable(tokens: &Vec<Token>) -> bool {
        tokens.len() == 1 && match Self::get_string_token_string(&tokens[0]) {
            Ok(s) => s == "variables",
            Err(_) => false
        }
    }

    pub fn is_show_functions(tokens: &Vec<Token>) -> bool {
        tokens.len() == 1 && match Self::get_string_token_string(&tokens[0]) {
            Ok(s) => s == "functions",
            Err(_) => false
        }
    }

    pub fn is_show_commands(tokens: &Vec<Token>) -> bool {
        tokens.len() == 1 && match Self::get_string_token_string(&tokens[0]) {
            Ok(s) => s == "history",
            Err(_) => false
        }
    }

    pub fn separate_equal(tokens: Vec<Token>) -> Result<(Vec<Token>, Vec<Token>), String> {
        let mut left_vec = Vec::new();
        let mut right_vec = Vec::new();
        let mut equal_flag = false;
        for token in tokens {
            if token == Token::Equal {
                if equal_flag {
                    return Err("= appeared twice: syntax error".to_string())
                }
                equal_flag = true;
            } else {
                if equal_flag {
                    right_vec.push(token);
                } else {
                    left_vec.push(token);
                }
            }
        }
        if !equal_flag {
            return Err("= never appeared: syntax error".to_string())
        }
        Ok((left_vec, right_vec))
    }

    pub fn is_question_tokens(tokens: &Vec<Token>) -> bool {
        tokens.len() == 1 && tokens[0] == Token::Question
    }

    pub fn is_solution_equation(tokens: &Vec<Token>) -> bool {
        tokens.len() > 0 && tokens.last() == Some(&Token::Question)
    }

    pub fn is_variable_register(tokens: &Vec<Token>) -> bool {
        tokens.len() == 1 && if let Token::String(_) = tokens[0] {true} else {false}
    }

    pub fn is_func_register(tokens: &Vec<Token>) -> bool {
        tokens.len() == 4
            && if let Token::String(_) = tokens[0] {true} else {false}
            && if let Token::LParen = tokens[1] {true} else {false}
            && if let Token::String(_) = tokens[2] {true} else {false}
            && if let Token::RParen = tokens[3] {true} else {false}
    }

    pub fn get_string_token_string(token: &Token) -> Result<&String, String> {
        match token {
            Token::String(s) => Ok(s),
            _ => Err("syntax error".to_string())
        }
    }

    pub fn get_num_token_float(token: &Token) -> Result<f64, String> {
        match token {
            Token::NumString(s) => {
                let num = Self::string_to_num(s)?;
                match num {
                    Num::Float(n) => Ok(n),
                    _ => Err("syntax error".to_string()),
                }
            }
            _ => Err("syntax error".to_string())
        }
    }

    pub fn make_tree(&mut self, data_base: &DataBase) -> Result<BinaryTree<Element>, String> {
        let mut tree = BinaryTree::new();
        self.root_tree_loop(&mut tree, data_base)?;
        if self.index < self.tokens.len() {
            return Err(format!("syntax error"))
        }
        Ok(tree)
    }

    fn root_tree_loop(&mut self, tree: &mut BinaryTree<Element>, data_base: &DataBase) -> Result<(), String> {
        while self.index < self.tokens.len() {
            self.search_add_point(tree, data_base)?;
            if self.is_r_paren() {
                break
            }
        }
        Ok(())
    }

    fn search_add_point(&mut self, tree: &mut BinaryTree<Element>, data_base: &DataBase) -> Result<(), String> {
        match tree {
            BinaryTree::Empty => {
                self.while_next_token(tree, data_base)?;
            },
            BinaryTree::NonEmpty(_) => {
                let token = self.get_next_token()?;
                match token {
                    Token::Plus | Token::Minus | Token::Asterisk | 
                        Token::Slash | Token::Percent | Token::Caret | Token::TwoAsterisk => {
                        let operator = Self::token_to_operator(token)?;
                        let tree_op = Self::get_tree_element_operator(tree)?;
                        if tree_op.priority(&operator) {
                            self.search_add_point(tree.right_mut().unwrap(), data_base)?;
                        } else {
                            Self::replace_and_add_left(tree, operator);
                            if !tree.left().unwrap().is_non_empty() {
                                return Err(format!("syntax error"))
                            }
                            self.index_plus();
                            self.while_next_token(tree, data_base)?;
                        }
                    },
                    _ => return Err(format!("syntax error")),
                }
            }
        }
        Ok(())
    }

    fn token_to_operator(token: &Token) -> Result<Operator, String> {
        match token {
            Token::Plus => Ok(Operator::Plus),
            Token::Minus => Ok(Operator::Minus),
            Token::Asterisk => Ok(Operator::Mul),
            Token::Slash => Ok(Operator::Div),
            Token::Percent => Ok(Operator::Rem),
            Token::Caret => Ok(Operator::Pow),
            Token::TwoAsterisk => Ok(Operator::MatrixMul),
            _ => Err(format!("syntax error")),
        }
    }

    fn get_tree_element_operator(tree: &BinaryTree<Element>) -> Result<&Operator, String> {
        match tree {
            BinaryTree::Empty => Err(format!("syntax error")),
            BinaryTree::NonEmpty(node_box) => {
                match &node_box.element {
                    Element::Operator(op) => Ok(op),
                    _ => Err(format!("syntax error")),
                }
            }
        }
    }

    fn get_next_token(&mut self) -> Result<&Token, String> {
        match self.tokens.get(self.index) {
            Some(v) => {
                Ok(v)
            },
            None => Err(format!("syntax error")),
        }
    }

    fn index_plus(&mut self) {
        self.index += 1;
    }

    fn while_next_token(&mut self, tree: &mut BinaryTree<Element>, data_base: &DataBase) -> Result<bool, String> {
        while self.index < self.tokens.len() {
            if self.next_token(tree, data_base)? {
                return Ok(true)
            }
        }
        Ok(false)
    }

    fn next_token(&mut self, tree: &mut BinaryTree<Element>, data_base: &DataBase) -> Result<bool, String> {
        let token = self.get_next_token()?;
        match token {
            Token::NumString(s) => {
                let num = Self::string_to_num(s)?;
                self.add_num(tree, num)?;
                self.index_plus();
            },
            Token::I => return self.add_complex(tree),
            Token::Plus => return self.add_operator(tree, Operator::Plus, data_base),
            Token::Minus => return self.add_operator(tree, Operator::Minus, data_base),
            Token::Asterisk => return self.add_operator(tree, Operator::Mul, data_base),
            Token::Slash => return self.add_operator(tree, Operator::Div, data_base),
            Token::Percent => return self.add_operator(tree, Operator::Rem, data_base),
            Token::Caret => return self.add_operator(tree, Operator::Pow, data_base),
            Token::TwoAsterisk => return self.add_operator(tree, Operator::MatrixMul, data_base),
            Token::LParen => return self.add_paren(tree, data_base),
            Token::LBracket => return self.add_matrix(tree),
            Token::RParen => return Ok(true),
            Token::String(s) => {
                let string_box = s.clone();
                if Self::is_function(&string_box, data_base) {
                    return self.add_function(tree, string_box, data_base)
                } else {
                    return self.add_variable(tree, string_box)
                }
            },
            _ => return Err(format!("syntax error")),
        }
        Ok(false)
    }

    fn is_function(string_box: &Box<String>, data_base: &DataBase) -> bool {
        match data_base.get_func(&string_box) {
            Some(_) => true,
            None => false,
        }
    }

    fn string_to_num(string: &String) -> Result<Num, String> {
        let num = Num::from_string_to_float(string)?;
        num.checked_value()?;
        Ok(num)
    }

    fn add_num(&mut self, tree: &mut BinaryTree<Element>, num: Num) -> Result<(), String> {
        let next_tree = match tree {
            BinaryTree::Empty => tree,
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Dummy | Element::Variable(_) | Element::Func(_)
                        => return Err(format!("{:?}: syntax error", num)),
                    _ => {},
                }
                let left_tree = tree.left_mut().unwrap();
                match left_tree {
                    BinaryTree::Empty => left_tree,
                    BinaryTree::NonEmpty(_) => {
                        let right_tree = tree.right_mut().unwrap();
                        if right_tree.is_non_empty() {
                            return Err(format!("{:?}: syntax error", num))
                        }
                        right_tree
                    }
                }
            }
        };
        *next_tree = BinaryTree::from_element(Element::Num(num));
        Ok(())
    }

    fn add_complex(&mut self, tree: &mut BinaryTree<Element>) -> Result<bool, String> {
        let next_tree = match tree {
            BinaryTree::Empty => tree,
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Variable(_) | Element::Func(_) => {
                        self.insert_mul();
                        return Ok(false)
                    }
                    Element::Dummy => return Err(format!("i: syntax error")),
                    Element::Operator(_) => {
                        if tree.right().unwrap().is_non_empty() {
                            self.insert_mul();
                            return Ok(false)
                        }
                    },
                }
                tree.right_mut().unwrap()
            }
        };
        *next_tree = BinaryTree::from_element(Element::Num(Num::new_complex()));
        self.index_plus();
        if self.is_num() || self.is_string_token() {
            self.insert_mul();
        }
        Ok(false)
    }

    fn add_matrix(&mut self, tree: &mut BinaryTree<Element>) -> Result<bool, String> {
        let next_tree = match tree {
            BinaryTree::Empty => tree,
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Variable(_) | Element::Func(_) => {
                        self.insert_mul();
                        return Ok(false)
                    }
                    Element::Dummy => return Err(format!("syntax error")),
                    Element::Operator(_) => {
                        if tree.right().unwrap().is_non_empty() {
                            self.insert_mul();
                            return Ok(false)
                        }
                    },
                }
                tree.right_mut().unwrap()
            }
        };
        *next_tree = BinaryTree::from_element(Element::Num(self.token_to_matrix()?));
        if self.is_num() || self.is_string_token() {
            self.insert_mul();
        }
        Ok(false)
    }

    fn token_to_matrix(&mut self) -> Result<Num, String> {
        let mut vec = Vec::new();
        self.index_plus();
        let flag = 'outer: loop {
            if !self.is_next_token(Token::LBracket) {
                break false
            }
            self.index_plus();
            let mut v = Vec::new();
            loop {
                if !self.is_num() {
                    break 'outer false
                }
                let value = Self::get_num_token_float(self.get_next_token()?)?;
                v.push(value);
                self.index_plus();
                if !self.is_next_token(Token::Comma) {
                    break
                }
                self.index_plus();
            }
            if !self.is_next_token(Token::RBracket) {
                break false
            }
            self.index_plus();
            vec.push(v);
            if !self.is_next_token(Token::SemiColon) {
                break true
            }
            self.index_plus();
        };
        if !flag || !self.is_next_token(Token::RBracket) {
            Err(format!("syntax error"))
        } else {
            self.index_plus();
            Num::from_vec(vec)
        }
    }

    fn is_next_token(&mut self, token: Token) -> bool {
        match self.get_next_token() {
            Ok(t) => {
                t == &token
            },
            Err(_) => false,
        }
    }

    fn add_operator(&mut self, tree: &mut BinaryTree<Element>, operator: Operator, data_base: &DataBase) -> Result<bool, String> {
        match &tree {
            BinaryTree::Empty => {
                match operator {
                    Operator::Plus | Operator::Minus => {
                        *tree = BinaryTree::from_element(Element::Operator(operator));
                        self.index_plus();
                        *tree.left_mut().unwrap() = BinaryTree::from_element(Element::Dummy);
                    },
                    _ => return Err(format!("Unsupported unary operators: syntax error"))
                }
            }
            BinaryTree::NonEmpty(node_box) => {
                match &node_box.element {
                    Element::Num(_) | Element::Variable(_) | Element::Func(_) => {
                        Self::replace_and_add_left(tree, operator);
                        self.index_plus();
                    },
                    Element::Operator(tree_op) => {
                        if !tree.right().unwrap().is_non_empty() {
                            return Err(format!("syntax error"))
                        }
                        if tree_op.priority(&operator) {
                            return self.while_next_token(tree.right_mut().unwrap(), data_base)
                        } else {
                            return Ok(true);
                        }
                    },
                    Element::Dummy => return Err(format!("syntax error"))
                }
            }
        }
        Ok(false)
    }

    fn add_paren(&mut self, tree: &mut BinaryTree<Element>, data_base: &DataBase) -> Result<bool, String> {
        let paren_tree = match tree {
            BinaryTree::Empty => {
                *tree = BinaryTree::from_element(Element::Operator(Operator::Paren));
                self.index_plus();
                self.root_tree_loop(tree.left_mut().unwrap(), data_base)?;
                tree
            },
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Variable(_) | Element::Func(_)=> {
                        self.insert_mul();
                        return Ok(false)
                    }
                    Element::Operator(_) => {
                        if tree.right().unwrap().is_non_empty() {
                            self.insert_mul();
                            return Ok(false)
                        }
                    },
                    Element::Dummy => return Err(format!("syntax error"))
                }
                let right_tree = tree.right_mut().unwrap();
                *right_tree = BinaryTree::from_element(Element::Operator(Operator::Paren));
                self.index_plus();
                self.root_tree_loop(right_tree.left_mut().unwrap(), data_base)?;
                right_tree
            }
        };
        if !paren_tree.left().unwrap().is_non_empty() {
            return Err(format!("{}: syntax error", "()"))
        }
        if self.is_r_paren() {
            *paren_tree.right_mut().unwrap() = BinaryTree::from_element(Element::Operator(Operator::RParen));
            self.index_plus();
            if self.is_num() || self.is_string_token() {
                self.insert_mul();
            }
        } else {
            return Err(format!("{}: syntax error", "("))
        }
        Ok(false)
    }

    fn add_variable(&mut self, tree: &mut BinaryTree<Element>, string_box: Box<String>) -> Result<bool, String> {
        let next_tree = match tree {
            BinaryTree::Empty => tree,
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Variable(_) | Element::Func(_) => {
                        self.insert_mul();
                        return Ok(false)
                    }
                    Element::Dummy => return Err(format!("{}: syntax error", string_box)),
                    Element::Operator(_) => {
                        if tree.right().unwrap().is_non_empty() {
                            self.insert_mul();
                            return Ok(false)
                        }
                    },
                }
                tree.right_mut().unwrap()
            }
        };
        *next_tree = BinaryTree::from_element(Element::Variable(string_box));
        self.index_plus();
        if self.is_num() || self.is_string_token() {
            self.insert_mul();
        }
        Ok(false)
    }

    fn add_function(&mut self, tree: &mut BinaryTree<Element>, string_box: Box<String>, data_base: &DataBase) -> Result<bool, String> {
        let next_tree = match tree {
            BinaryTree::Empty => tree,
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Variable(_) | Element::Func(_) => {
                        self.insert_mul();
                        return Ok(false)
                    }
                    Element::Dummy => return Err(format!("{}: syntax error", string_box)),
                    Element::Operator(_) => {
                        if tree.right().unwrap().is_non_empty() {
                            self.insert_mul();
                            return Ok(false)
                        }
                    },
                }
                tree.right_mut().unwrap()
            }
        };
        *next_tree = BinaryTree::from_element(Element::Func(string_box.clone()));
        self.index_plus();
        if !self.is_l_paren() {
            return Err(format!("error: {} is defined as a function, so it needs parentheses", string_box))
        }
        let mut paren_tree = next_tree.left_mut().unwrap();
        self.add_paren(&mut paren_tree, data_base)?;
        *next_tree.right_mut().unwrap() = BinaryTree::from_element(Element::Operator(Operator::RParen));
        Ok(false)
    }

    fn insert_mul(&mut self) {
        self.tokens.insert(self.index, Token::Asterisk)
    }

    fn replace_and_add_left(tree: &mut BinaryTree<Element>, operator: Operator) {
        let tmp_tree
            = std::mem::replace(tree,
                    BinaryTree::from_element(Element::Operator(operator)));
        tree.add_left_node_from_tree(tmp_tree);
    }

    fn is_r_paren(&mut self) -> bool {
        match self.get_next_token() {
            Ok(token) => {
                match token {
                    Token::RParen => true,
                    _ => false,
                }
            },
            Err(_) => false,
        }
    }

    fn is_l_paren(&mut self) -> bool {
        match self.get_next_token() {
            Ok(token) => {
                match token {
                    Token::LParen => true,
                    _ => false,
                }
            },
            Err(_) => false,
        }
    }

    fn is_num(&mut self) -> bool {
        match self.get_next_token() {
            Ok(token) => {
                match token {
                    Token::NumString(_) => true,
                    _ => false,
                }
            },
            Err(_) => false,
        }
    }

    fn is_string_token(&mut self) -> bool {
        match self.get_next_token() {
            Ok(token) => {
                match token {
                    Token::String(_) => true,
                    _ => false,
                }
            },
            Err(_) => false,
        }
    }

    pub fn calculation(&self, tree: &mut BinaryTree<Element>, data_base: &DataBase, local_variable: Option<(&String, Option<&Data>)>) -> Result<Option<Num>, String> {
        match &tree {
            BinaryTree::Empty => return Err(format!("syntax error")),
            BinaryTree::NonEmpty(node_box) => {
                let op = match &node_box.element {
                    Element::Operator(Operator::RParen) => return Ok(Some(Num::Float(0.0))),
                    Element::Operator(op) => {
                        op.clone()
                    },
                    Element::Num(n) => return Ok(Some(n.clone())),
                    Element::Dummy => return Ok(Some(Num::Float(0.0))),
                    Element::Variable(string_box) => {
                        if let Some((key, data)) = local_variable {
                            if *key == **string_box {
                                match data {
                                    Some(d) => match d {
                                        Data::Num(n) => return Ok(Some(n.clone())),
                                        Data::Func(f) => {
                                            *tree = f.0.clone();
                                            return Ok(None)
                                        },
                                    },
                                    None => return Ok(None)
                                }
                            }
                        }
                        match data_base.get_num(string_box) {
                            None => return Ok(None),
                            Some(num) => {
                                *tree = BinaryTree::from_element(Element::Num(num.clone()));
                                return Ok(Some(num.clone()))
                            }
                        }
                    },
                    Element::Func(string_box) => {
                        match data_base.get_builtin_func(string_box) {
                            None => {},
                            Some(b) => {
                                let mut func_tree = b.0.clone();
                                let variable = b.1.clone();
                                let function_name = *string_box.clone();
                                let left_value = match self.calculation(tree.left_mut().unwrap(), data_base, None)? {
                                    None => Data::Func(Box::new((tree.left().unwrap().clone(), variable.clone()))),
                                    Some(num) => return Ok(Some(builtin_func(function_name, &num)?)),
                                };
                                match self.calculation(&mut func_tree.left_mut().unwrap(), data_base, Some((&variable, Some(&left_value))))? {
                                    None => {
                                        *tree = func_tree;
                                        return Ok(None)
                                    },
                                    Some(_) => return Err(format!("syntax error")),
                                }
                            }
                        }
                        match data_base.get_func(string_box) {
                            None => return Ok(None),
                            Some(b) => {
                                let mut func_tree = b.0.clone();
                                let variable = b.1.clone();
                                let left_value = match self.calculation(tree.left_mut().unwrap(), data_base, None)? {
                                    None => Data::Func(Box::new((tree.left().unwrap().clone(), variable.clone()))),
                                    Some(num) => Data::Num(num),
                                };
                                match self.calculation(&mut func_tree, data_base, Some((&variable, Some(&left_value))))? {
                                    None => {
                                        *tree = BinaryTree::from_element_and_tree(
                                            Element::Operator(Operator::Paren),
                                            func_tree,
                                            BinaryTree::from_element(Element::Operator(Operator::RParen)),
                                        );
                                        return Ok(None)
                                    },
                                    Some(num) => {
                                        *tree = BinaryTree::from_element(Element::Num(num.clone()));
                                        return Ok(Some(num))
                                    },
                                }
                            }
                        }
                    }
                };

                let left_tree = tree.left().unwrap();
                let right_tree = tree.right().unwrap();
                match (left_tree, right_tree) {
                    (BinaryTree::NonEmpty(_), BinaryTree::NonEmpty(_)) => {}
                    _ => return Err(format!("syntax error")),
                };
                let left_value_option =
                    self.calculation(tree.left_mut().unwrap(), data_base, local_variable)?;
                let right_value_option =
                    self.calculation(tree.right_mut().unwrap(), data_base, local_variable)?;
                let value = match (left_value_option, right_value_option) {
                    (Some(left_value), Some(right_value)) => {
                        match op {
                            Operator::Plus => left_value.supported_add(&right_value)?,
                            Operator::Minus => left_value.supported_sub(&right_value)?,
                            Operator::Mul => left_value.supported_mul(&right_value)?,
                            Operator::Div => left_value.supported_div(&right_value)?,
                            Operator::Rem => left_value.supported_rem(&right_value)?,
                            Operator::Pow => left_value.supported_pow(&right_value)?,
                            Operator::Paren => left_value,
                            Operator::MatrixMul => left_value.supported_matrix_mul(&right_value)?,
                            _ => return Err(format!("syntax error")),
                        }
                    },
                    (None, Some(right_value)) => {
                        Self::check_and_add_paren_to_value(tree, &right_value, &op, true)?;
                        return Ok(None)
                    },
                    (Some(left_value), None) => {
                        Self::check_and_add_paren_to_value(tree, &left_value, &op, false)?;
                        return Ok(None)
                    },
                    _ => return Ok(None)
                };
                value.checked_value()?;
                *tree = BinaryTree::from_element(Element::Num(value.clone()));
                return Ok(Some(value))
            }
        }
    }

    fn check_and_add_paren_to_value(tree: &mut BinaryTree<Element>, value: &Num, op: &Operator, right: bool) -> Result<(), String> {
        if !value.is_need_paren_to_display() {
            return Ok(())
        }
        match op {
            Operator::Minus | Operator::Plus => {
                if right && value.is_need_sign_reverse() {
                    let num = value.reverse_sign();
                    *tree.right_mut().unwrap() = BinaryTree::from_element(Element::Num(num.clone()));
                    if let Operator::Minus = op {
                        tree.set_element(Element::Operator(Operator::Plus));
                    } else {
                        tree.set_element(Element::Operator(Operator::Minus));
                        match &num {
                            Num::Complex(n) => {
                                if n.z != 0.0 {
                                    Self::add_paren_to_value(tree, &num, right);
                                }
                            },
                            Num::Float(_) | Num::Matrix(_) => {}
                        }
                    }
                }
            },
            Operator::Mul | Operator::Div | Operator::Rem | Operator::Pow | Operator::MatrixMul
                => Self::add_paren_to_value(tree, value, right),
            Operator::Paren | Operator::RParen => return Err(format!("syntax error"))
        }
        Ok(())
    }

    fn add_paren_to_value(tree: &mut BinaryTree<Element>, value: &Num, right: bool) {
        let tmp_tree = if right {tree.right_mut().unwrap()} else {tree.left_mut().unwrap()};
        *tmp_tree = BinaryTree::from_element_and_tree(
            Element::Operator(Operator::Paren),
            BinaryTree::from_element(Element::Num(value.clone())),
            BinaryTree::from_element(Element::Operator(Operator::RParen)),
        );
    }

    pub fn print_tree(tree: &BinaryTree<Element>) -> Result<String, String> {
        let mut expr = String::new();
        Self::print_tree_loop(tree, &mut expr)?;
        expr.pop();
        Ok(expr)
    }

    pub fn print_tree_loop(tree: &BinaryTree<Element>, expr: &mut String) -> Result<(), String> {
        match tree {
            BinaryTree::Empty => return Err(format!("syntax error")),
            BinaryTree::NonEmpty(node_box) => {
                match &node_box.element {
                    Element::Operator(op) => {
                        if let Operator::RParen = op {
                            *expr += format!("{} ", op).as_str();
                            return Ok(())
                        } else if let Operator::Paren = op {
                            *expr += format!("{} ", op).as_str();
                        }
                        let left_tree = tree.left().unwrap();
                        let right_tree = tree.right().unwrap();
                        match (left_tree, right_tree) {
                            (BinaryTree::NonEmpty(_), BinaryTree::NonEmpty(_)) => {
                                Self::print_tree_loop(left_tree, expr)?;
                                match op {
                                    Operator::RParen | Operator::Paren => {},
                                    _ => *expr += format!("{} ", op).as_str(),
                                }
                                Self::print_tree_loop(right_tree, expr)?;
                            },
                            _ => return Err(format!("syntax error")),
                        }
                    },
                    Element::Num(n) => *expr += format!("{} ", n).as_str(),
                    Element::Dummy => {},
                    Element::Variable(v) => *expr += format!("{} ", v).as_str(),
                    Element::Func(f) => {
                        *expr += format!("{} ", f).as_str();
                        let left_tree = tree.left().unwrap();
                        match left_tree {
                            BinaryTree::NonEmpty(_) => Self::print_tree_loop(left_tree, expr)?,
                            _ => return Err(format!("syntax error")),
                        }
                    },
                }
            },
        }
        Ok(())
    }

    pub fn check_variable_in_tree(tree: &BinaryTree<Element>) -> Result<Option<String>, String> {
        match tree {
            BinaryTree::Empty => return Ok(None),
            BinaryTree::NonEmpty(node_box) => {
                let left = Self::check_variable_in_tree(tree.left().unwrap())?;
                let right = Self::check_variable_in_tree(tree.right().unwrap())?;
                let variable = match (left, right) {
                    (Some(left_variable), Some(right_variable)) => {
                        if left_variable != right_variable {
                            return Err(format!("{}, {}: error two variable", left_variable, right_variable))
                        }
                        Some(left_variable)
                    },
                    (Some(left_variable), _) => Some(left_variable),
                    (_, Some(right_variable)) => Some(right_variable),
                    _ => None
                };
                let var = match &node_box.element {
                    Element::Dummy | Element::Num(_) | Element::Operator(_) => None,
                    Element::Variable(v) => Some(v),
                    Element::Func(v) => return Err(format!("{}: error function", v))
                };
                let variable = match (variable, var) {
                    (Some(string1), Some(string2)) => {
                        if **string2 != string1 {
                            return Err(format!("{}, {}: error two variable", string2, string1))
                        }
                        Some(string1)
                    },
                    (Some(string1), _) => Some(string1),
                    (_, Some(string2)) => Some(*string2.clone()),
                    _ => None,
                };
                Ok(variable)
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn calculation_test(code: String) -> Result<Num, String> {
        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return Err(format!("error lexer: {}", e))
        };
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register_num(&name, Num::Float(2.0))?;
        let name = "y".to_string();
        data_base.register_num(&name, Num::Float(-2.0))?;
        let name = "z".to_string();
        data_base.register_num(&name, Num::from_two_float_to_complex(-1.0, -3.0))?;
        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree(&data_base) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parser: {}", e))
        };
        match parser.calculation(&mut tree, &data_base, None) {
            Ok(v) => {
                match v {
                    Some(v) => Ok(v),
                    None => Err(format!("error calculation"))
                }
            },
            Err(e) => Err(format!("error calculation: {}", e))
        }
    }

    fn parse_and_print_test(code: String) -> String {
        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return format!("error lexer: {}", e)
        };
        let data_base = DataBase::new();
        let mut parser = Parser::new(vec);
        match parser.make_tree(&data_base) {
            Ok(v) => {
                let tree = v;
                match Parser::print_tree(&tree) {
                    Ok(s) => s,
                    Err(e) => format!("error print_tree: {}", e),
                }
            },
            Err(e) => format!("error parser: {}", e)
        }
    }

    fn calculation_and_print_test(code: String) -> String {
        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return format!("error lexer: {}", e)
        };
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register_num(&name, Num::Float(2.0)).unwrap();
        let name = "y".to_string();
        data_base.register_num(&name, Num::Float(-2.0)).unwrap();
        let name = "z".to_string();
        data_base.register_num(&name, Num::from_two_float_to_complex(-1.0, -3.0)).unwrap();
        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree(&data_base) {
            Ok(v) => v,
            Err(e) => return format!("error parser: {}", e)
        };
        match parser.calculation(&mut tree, &data_base, None) {
            Ok(_) => {
                match Parser::print_tree(&tree) {
                    Ok(s) => s,
                    Err(e) => format!("error print_tree: {}", e),
                }
            }
            Err(e) => format!("error calculation: {}", e)
        }
    }

    fn check_variable_in_tree_test(code: String) -> String {
        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return format!("error lexer: {}", e)
        };
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register_num(&name, Num::Float(2.0)).unwrap();
        let name = "y".to_string();
        data_base.register_num(&name, Num::Float(-2.0)).unwrap();
        let name = "z".to_string();
        data_base.register_num(&name, Num::from_two_float_to_complex(-1.0, -3.0)).unwrap();
        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree(&data_base) {
            Ok(v) => v,
            Err(e) => return format!("error parser: {}", e)
        };
        match parser.calculation(&mut tree, &data_base, None) {
            Ok(_) => {
                match Parser::check_variable_in_tree(&tree) {
                    Ok(s) => {
                        match s {
                            Some(v) => v,
                            None => format!("None"),
                        }
                    },
                    Err(e) => format!("error check_variable: {}", e),
                }
            }
            Err(e) => format!("error calculation: {}", e)
        }
    }


    fn function_calculation_test(function: String, function_name: String, variable: String, code: String) -> Result<Num, String> {
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register_num(&name, Num::Float(2.0)).unwrap();
        let name = "y".to_string();
        data_base.register_num(&name, Num::Float(-2.0)).unwrap();
        let name = "z".to_string();
        data_base.register_num(&name, Num::from_two_float_to_complex(-1.0, -3.0)).unwrap();

        let mut lexer = Lexer::new(&function);
        let vec = lexer.make_token_vec()?;
        let mut parser = Parser::new(vec);
        let mut tree = parser.make_tree(&data_base)?;
        parser.calculation(&mut tree, &data_base, Some((&variable, None)))?;

        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return Err(format!("error lexer: {}", e))
        };
        match Parser::check_variable_in_tree(&tree)? {
            Some(var) => {
                if var != *variable {
                    return Err(format!("{}, {}: error two variable", var, variable))
                }
            },
            None => {},
        }
        data_base.register_func(&function_name, tree, variable.clone())?;

        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree(&data_base) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parser: {}", e))
        };
        match parser.calculation(&mut tree, &data_base, None) {
            Ok(v) => {
                match v {
                    Some(v) => Ok(v),
                    None => Err(format!("error calculation"))
                }
            },
            Err(e) => Err(format!("error calculation: {}", e))
        }
    }


    fn function_calculation_tree_test(function: String, function_name: String, variable: String, code: String) -> Result<String, String> {
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register_num(&name, Num::Float(2.0)).unwrap();
        let name = "y".to_string();
        data_base.register_num(&name, Num::Float(-2.0)).unwrap();
        let name = "z".to_string();
        data_base.register_num(&name, Num::from_two_float_to_complex(-1.0, -3.0)).unwrap();

        let mut lexer = Lexer::new(&function);
        let vec = lexer.make_token_vec()?;
        let mut parser = Parser::new(vec);
        let mut tree = parser.make_tree(&data_base)?;
        parser.calculation(&mut tree, &data_base, Some((&variable, None)))?;

        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return Err(format!("error lexer: {}", e))
        };
        match Parser::check_variable_in_tree(&tree)? {
            Some(var) => {
                if var != *variable {
                    return Err(format!("{}, {}: error two variable", var, variable))
                }
            },
            None => {},
        }
        data_base.register_func(&function_name, tree, variable.clone())?;

        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree(&data_base) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parser: {}", e))
        };
        match parser.calculation(&mut tree, &data_base, None) {
            Ok(_) => {},
            Err(e) => return Err(format!("error calculation: {}", e))
        }
        match Parser::print_tree(&tree) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("error print_tree: {}", e)),
        }
    }


    #[test]
    fn calculation_simple_float_add() {
        let code = "1 + 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(3.0)))
    }

    #[test]
    fn calculation_simple_float_mul() {
        let code = "2 * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(6.0)))
    }

    #[test]
    fn calculation_simple_float_sub() {
        let code = "2 - 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_simple_float_div() {
        let code = "3 / 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.5)))
    }

    #[test]
    fn calculation_simple_float_rem() {
        let code = "4 % 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_simple_float_pow() {
        let code = "4^2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(16.0)))
    }

    #[test]
    fn calculation_simple_float_add_sub() {
        let code = "1 + 2 - 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_simple_float_add_mul() {
        let code = "1 + 2 * 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(9.0)))
    }

    #[test]
    fn calculation_simple_float_add_div() {
        let code = "1 + 2 / 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.5)))
    }

    #[test]
    fn calculation_simple_float_add_add() {
        let code = "1 + 2 + 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(7.0)))
    }

    #[test]
    fn calculation_simple_float_add_rem() {
        let code = "1 + 5 % 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(3.0)))
    }

    #[test]
    fn calculation_simple_float_add_pow() {
        let code = "1 + 2 ^ 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(17.0)))
    }

    #[test]
    fn calculation_simple_float_sub_add() {
        let code = "1 - 2 + 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(3.0)))
    }

    #[test]
    fn calculation_simple_float_sub_sub() {
        let code = "1 - 2 - 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-5.0)))
    }

    #[test]
    fn calculation_simple_float_sub_mul() {
        let code = "1 - 2 * 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-7.0)))
    }

    #[test]
    fn calculation_simple_float_sub_div() {
        let code = "1 - 2 / 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(0.5)))
    }

    #[test]
    fn calculation_simple_float_sub_rem() {
        let code = "1 - 5 % 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_simple_float_sub_pow() {
        let code = "1 - 2 ^ 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-7.0)))
    }

    #[test]
    fn calculation_simple_float_mul_add() {
        let code = "2 * 3 + 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(10.0)))
    }

    #[test]
    fn calculation_simple_float_mul_sub() {
        let code = "2 * 3 - 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(2.0)))
    }

    #[test]
    fn calculation_simple_float_mul_mul() {
        let code = "2 * 3 * 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(24.0)))
    }

    #[test]
    fn calculation_simple_float_mul_div() {
        let code = "2 * 3 / 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.5)))
    }

    #[test]
    fn calculation_simple_float_mul_rem() {
        let code = "2 * 5 % 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_simple_float_mul_pow() {
        let code = "3 * 2 ^ 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(24.0)))
    }

    #[test]
    fn calculation_simple_float_div_add() {
        let code = "4 / 2 + 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(5.0)))
    }

    #[test]
    fn calculation_simple_float_div_sub() {
        let code = "4 / 2 - 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_simple_float_div_mul() {
        let code = "4 / 2 * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(6.0)))
    }

    #[test]
    fn calculation_simple_float_div_div() {
        let code = "4 / 2 / 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(0.5)))
    }

    #[test]
    fn calculation_simple_float_div_rem() {
        let code = "8 / 2 % 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_simple_float_div_pow() {
        let code = "8 / 2 ^ 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_simple_float_rem_add() {
        let code = "5 % 3 + 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(5.0)))
    }

    #[test]
    fn calculation_simple_float_rem_sub() {
        let code = "5 % 3 - 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_simple_float_rem_mul() {
        let code = "5 % 3 * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(6.0)))
    }

    #[test]
    fn calculation_simple_float_rem_div() {
        let code = "5 % 3 / 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(0.5)))
    }

    #[test]
    fn calculation_simple_float_rem_rem() {
        let code = "5 % 3 % 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(0.0)))
    }

    #[test]
    fn calculation_simple_float_rem_pow() {
        let code = "10 % 3 ^ 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_simple_float_pow_add() {
        let code = "2 ^ 3 + 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(11.0)))
    }

    #[test]
    fn calculation_simple_float_pow_sub() {
        let code = "2 ^ 3 - 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(5.0)))
    }

    #[test]
    fn calculation_simple_float_pow_mul() {
        let code = "2 ^ 3 * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(24.0)))
    }

    #[test]
    fn calculation_simple_float_pow_div() {
        let code = "2 ^ 3 / 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(4.0)))
    }

    #[test]
    fn calculation_simple_float_pow_rem() {
        let code = "2 ^ 3 % 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(2.0)))
    }

    #[test]
    fn calculation_simple_float_pow_pow() {
        let code = "2 ^ 3 ^ 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(512.0)))
    }

    #[test]
    fn calculation_multi_priority() {
        let code = "1 + 2^3 * 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(17.0)))
    }

    #[test]
    fn calculation_double_priority() {
        let code = "1 - 2 * 2 * 2 + 5".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-2.0)))
    }

    #[test]
    fn calculation_long() {
        let code = "1 + 2 * 3 - 8 / 2 % 3 + 2 ^ 3 * 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(22.0)))
    }

    #[test]
    fn calculation_complex_long() {
        let code = "i + 2 * 3 - 8 / 2i % 3 + 2 ^ 3 * i * 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::from_two_float(6.0, 16.0)))
    }

    #[test]
    fn calculation_matrix_long() -> Result<(), String> {
        let code = "[[1,2]] [[2,1]] ** [[3, 3, 3];[3, 3, 3]]".to_string();
        let num = Num::from_vec(vec![vec![12.0; 3]])?;
        assert_eq!(calculation_test(code), Ok(num));
        Ok(())
    }

    #[test]
    fn calculation_error_matrix_plus() -> Result<(), String> {
        let code = "2 + [[3, 3, 3];[3, 3, 3]]".to_string();
        assert_eq!(calculation_test(code), Err(format!("error calculation: Unsupported operator 2 + [[3,3,3];[3,3,3]]")));
        Ok(())
    }

    #[test]
    fn calculation_error_matrix_size() -> Result<(), String> {
        let code = "[[3, 3];[3, 3]] + [[3, 3, 3];[3, 3, 3]]".to_string();
        assert_eq!(calculation_test(code), Err(format!("error calculation: Unsupported different sizes operator [[3,3];[3,3]] + [[3,3,3];[3,3,3]]")));
        Ok(())
    }

    #[test]
    fn calculation_error_complex_rem() {
        let code = "3 % i".to_string();
        assert_eq!(calculation_test(code), Err("error calculation: Unsupported operator (3) % (i)".to_string()))
    }

    #[test]
    fn calculation_error_double_op() {
        let code = "1 + + 2".to_string();
        assert_eq!(calculation_test(code), Err("error parser: syntax error".to_string()))
    }

    #[test]
    fn calculation_error_double_num() {
        let code = "2 3".to_string();
        assert_eq!(calculation_test(code), Err("error parser: Float(3.0): syntax error".to_string()))
    }

    #[test]
    fn calculation_error_double_op_double_num() {
        let code = "1 + + 2 3".to_string();
        assert_eq!(calculation_test(code), Err("error parser: syntax error".to_string()))
    }

    #[test]
    fn calculation_paren_single() {
        let code = "(1+1)".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(2.0)))
    }

    #[test]
    fn calculation_paren_solo() {
        let code = "(1)".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_paren_priority_plus() {
        let code = "1 - (2 + 1)".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-2.0)))
    }

    #[test]
    fn calculation_paren_priority_mul() {
        let code = "1 - 2 * (2 + 1)".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-5.0)))
    }

    #[test]
    fn calculation_paren_priority_pow() {
        let code = "1 - 2 ^ (2 + 1) * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-23.0)))
    }

    #[test]
    fn calculation_paren_first() {
        let code = "(1 + 1) * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(6.0)))
    }

    #[test]
    fn calculation_paren_repeat() {
        let code = "(1 - (2 * 3)) * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-15.0)))
    }

    #[test]
    fn calculation_paren_left_no_mul() {
        let code = "2 + 3 (3 - 1) + 5".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(13.0)))
    }

    #[test]
    fn calculation_paren_left_no_mul_pow() {
        let code = "2 + 2 ^ 3 (3 - 1) + 5".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(23.0)))
    }

    #[test]
    fn calculation_paren_left_no_mul_after_pow() {
        let code = "2 + 3 (3 - 1) ^ 2 + 5".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(19.0)))
    }

    #[test]
    fn calculation_paren_right_no_mul() {
        let code = "2 + (3 - 1)3 + 5".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(13.0)))
    }

    #[test]
    fn calculation_paren_right_no_mul_pow() {
        let code = "2 + (3 - 1) 2 ^ 3 + 5".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(23.0)))
    }


    #[test]
    fn calculation_paren_priority_long() {
        let code = "1 - 2 ^ (2 + 1) * 3 + 2 * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-17.0)))
    }

    #[test]
    fn calculation_paren_complex_long() {
        let code = "1 - 2 ^ (2 + 1) * i + i * 3".to_string();
        assert_eq!(calculation_test(code), Ok(Num::from_two_float(1.0, -5.0)))
    }

    #[test]
    fn calculation_error_paren_only() {
        let code = "1 + ()".to_string();
        assert_eq!(calculation_test(code), Err("error parser: (): syntax error".to_string()))
    }

    #[test]
    fn calculation_error_l_paren_only() {
        let code = "1 + ( 3 + 4".to_string();
        assert_eq!(calculation_test(code), Err("error parser: (: syntax error".to_string()))
    }

    #[test]
    fn calculation_error_r_paren_only() {
        let code = "1 + ) 3 + 4".to_string();
        assert_eq!(calculation_test(code), Err("error parser: syntax error".to_string()))
    }

    #[test]
    fn calculation_float_unary_plus() {
        let code = "+ 2 * 3 + 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(10.0)))
    }

    #[test]
    fn calculation_float_unary_minus() {
        let code = "- 2 * 3 + 4".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-2.0)))
    }

    #[test]
    fn calculation_float_unary_paren() {
        let code = "+ 2 * 3 * ( -2 + 1 ) + 1".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-5.0)))
    }

    #[test]
    fn calculation_error_float_unary_mul() {
        let code = "* 2 * 3 * ( -2 + 1 ) + 1".to_string();
        assert_eq!(calculation_test(code), Err("error parser: Unsupported unary operators: syntax error".to_string()))
    }

    #[test]
    fn calculation_variable_normal() {
        let code = "x + 1".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(3.0)))
    }

    #[test]
    fn calculation_variable_upper() {
        let code = "X + 1".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(3.0)))
    }

    #[test]
    fn calculation_double_variable() {
        let code = "x + y + 1".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(1.0)))
    }

    #[test]
    fn calculation_variable_chain() {
        let code = "2x y 3 + 1".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(-23.0)))
    }

    #[test]
    fn calculation_variable_paren() {
        let code = "2 x y (1 - 2) + 1".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(9.0)))
    }

    #[test]
    fn calculation_variable_in_paren() {
        let code = "2 (x)".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(4.0)))
    }

    #[test]
    fn calculation_variable_last() {
        let code = "2 + x".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(4.0)))
    }

    #[test]
    fn calculation_error_variable_not_found() {
        let code = "2a + 1".to_string();
        assert_eq!(calculation_test(code), Err("error calculation".to_string()))
    }

    #[test]
    fn print_tree_normal() {
        let code = "2x + 1".to_string();
        assert_eq!(parse_and_print_test(code), format!("2 * x + 1"))
    }

    #[test]
    fn print_tree_long() {
        let code = "- 1 + 2 (x + y) ^ 2 * 3 - 2x".to_string();
        assert_eq!(parse_and_print_test(code), format!("- 1 + 2 * ( x + y ) ^ 2 * 3 - 2 * x"))
    }

    #[test]
    fn print_tree_complex_long() {
        let code = "- 1 + 2i (x + y) ^ 2 * 3 - 2i x + 3x".to_string();
        assert_eq!(parse_and_print_test(code), format!("- 1 + 2 * i * ( x + y ) ^ 2 * 3 - 2 * i * x + 3 * x"))
    }

    #[test]
    fn calculation_and_print_num() {
        let code = "- 1 + 2 (x + y) ^ 2 * 3 - 2x".to_string();
        assert_eq!(calculation_and_print_test(code), format!("-5"))
    }

    #[test]
    fn calculation_and_print_ab() {
        let code = "- 1 + 2 (a + b) ^ 2 * 3 - 2a".to_string();
        assert_eq!(calculation_and_print_test(code), format!("-1 + 2 * ( a + b ) ^ 2 * 3 - 2 * a"))
    }

    #[test]
    fn calculation_and_print_minus_minus() {
        let code = "a - y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a + 2"))
    }

    #[test]
    fn calculation_and_print_minus_complex() {
        let code = "a - z".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a + 1 + 3i"))
    }

    #[test]
    fn calculation_and_print_plus_minus() {
        let code = "a + y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a - 2"))
    }

    #[test]
    fn calculation_and_print_plus_complex() {
        let code = "a + z".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a - ( 1 + 3i )"))
    }

    #[test]
    fn calculation_and_print_mul_minus() {
        let code = "a * y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a * ( -2 )"))
    }

    #[test]
    fn calculation_and_print_mul_complex() {
        let code = "a * z".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a * ( -1 - 3i )"))
    }

    #[test]
    fn calculation_and_print_minus_mul() {
        let code = "y ^ a".to_string();
        assert_eq!(calculation_and_print_test(code), format!("( -2 ) ^ a"))
    }

    #[test]
    fn calculation_and_print_pow_complex() {
        let code = "z ^ a".to_string();
        assert_eq!(calculation_and_print_test(code), format!("( -1 - 3i ) ^ a"))
    }

    #[test]
    fn calculation_and_print_axy() {
        let code = "- 1 + 2 (x + a) ^ 2 * 3 - 2y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("-1 + 2 * ( 2 + a ) ^ 2 * 3 + 4"))
    }

    #[test]
    fn check_variable_in_tree_normal() {
        let code = "- 1 + 2 (x + a) ^ 2 * 3 - 2y".to_string();
        assert_eq!(check_variable_in_tree_test(code), format!("a"))
    }

    #[test]
    fn check_variable_in_tree_complex() {
        let code = "- 1 + 2 (x + a) ^ 2 * 3i - 2y".to_string();
        assert_eq!(check_variable_in_tree_test(code), format!("a"))
    }

    #[test]
    fn check_variable_in_tree_small() {
        let code = "a".to_string();
        assert_eq!(check_variable_in_tree_test(code), format!("a"))
    }

    #[test]
    fn calculation_function_normal() {
        let function = "a + 1".to_string();
        let function_name = "func".to_string();
        let variable = "a".to_string();
        let code = "func(1)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(2.0)))
    }

    #[test]
    fn calculation_function_complex() {
        let function = "a + i".to_string();
        let function_name = "func".to_string();
        let variable = "a".to_string();
        let code = "func(2 + i)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::from_two_float(2.0, 2.0)))
    }

    #[test]
    fn calculation_function_matrix() {
        let function = "a ** [[1,1]]".to_string();
        let function_name = "func".to_string();
        let variable = "a".to_string();
        let code = "func([[1];[1];[1]])".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::from_vec(vec![vec![1.0;2];3]).unwrap()))
    }

    #[test]
    fn calculation_function_registered_variable() {
        let function = "x + 1".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func(1)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(2.0)))
    }

    #[test]
    fn calculation_function_constant() {
        let function = "-1".to_string();
        let function_name = "func".to_string();
        let variable = "a".to_string();
        let code = "func(1)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_function_long_function() {
        let function = "(x + 2)^2 - 2 * x - 2 + (-2)^x".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func(2)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(14.0)))
    }

    #[test]
    fn calculation_function_in_same_variable() {
        let function = "(x + 2)^2 - 2 * x - 2 + (-2)^x".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func(x * 2 - 2)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(14.0)))
    }

    #[test]
    fn calculation_function_in_long() {
        let function = "(x + 2)^2 - 2 * x - 2 + (-2)^x".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func((x + 2)^2 - 2 * x - 2 + (-2)^x - 12)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(14.0)))
    }

    #[test]
    fn calculation_function_and_long() {
        let function = "x + 1".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "3 func(x) x - 2func(y)^3".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(20.0)))
    }

    #[test]
    fn calculation_function_overwrite_variable() {
        let function = "x + 1".to_string();
        let function_name = "x".to_string();
        let variable = "x".to_string();
        let code = "x(-2)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Ok(Num::Float(-1.0)))
    }

    #[test]
    fn calculation_error_function_not_func() {
        let function = "1 - x".to_string();
        let function_name = "f".to_string();
        let variable = "x".to_string();
        let code = "func(1)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Err("error calculation".to_string()))
    }

    #[test]
    fn calculation_error_function_not_paren() {
        let function = "1 - x".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Err("error parser: error: func is defined as a function, so it needs parentheses".to_string()))
    }

    #[test]
    fn calculation_error_function_unsupported_op_complex() {
        let function = "1 - i ^ x".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func(2)".to_string();
        assert_eq!(function_calculation_test(function, function_name, variable, code), Err("error calculation: Unsupported operator (i) ^ (2)".to_string()))
    }

    #[test]
    fn calculation_function_tree_variable() {
        let function = "(x + 2)^2 - 2 * x - 2 + (-2)^x".to_string();
        let function_name = "func".to_string();
        let variable = "x".to_string();
        let code = "func(a)".to_string();
        let ans = "( ( ( a ) + 2 ) ^ 2 - 2 * ( a ) - 2 + ( -2 ) ^ ( a ) )".to_string();
        assert_eq!(function_calculation_tree_test(function, function_name, variable, code), Ok(ans))
    }
}
