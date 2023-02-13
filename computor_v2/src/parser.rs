use crate::num::Num;
use crate::binary_tree::{BinaryTree, TreeNode};
use crate::lexer::Token;
use crate::operator::Operator;
use crate::data_base::DataBase;


#[derive(Debug, PartialEq)]
pub enum Element {
    Dummy,
    Operator(Operator),
    Num(Num),
    Variable(Box<String>),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
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

    pub fn is_variable_register(tokens: &Vec<Token>) -> bool {
        tokens.len() == 1 && if let Token::String(_) = tokens[0] {true} else {false}
    }

    pub fn get_string_token_string(token: &Token) -> Result<&String, String> {
        match token {
            Token::String(s) => Ok(s),
            _ => Err("syntax error".to_string())
        }
    }

    pub fn make_tree(&mut self) -> Result<BinaryTree<Element>, String> {
        let mut tree = BinaryTree::new();
        self.root_tree_loop(&mut tree)?;
        if self.index < self.tokens.len() {
            return Err(format!("syntax error"))
        }
        Ok(tree)
    }

    fn root_tree_loop(&mut self, tree: &mut BinaryTree<Element>) -> Result<(), String> {
        while self.index < self.tokens.len() {
            self.search_add_point(tree)?;
            if self.is_r_paren() {
                break
            }
        }
        Ok(())
    }

    fn search_add_point(&mut self, tree: &mut BinaryTree<Element>) -> Result<(), String> {
        match tree {
            BinaryTree::Empty => {
                self.while_next_token(tree)?;
            },
            BinaryTree::NonEmpty(_) => {
                let token = self.get_next_token()?;
                match token {
                    Token::Plus | Token::Minus | Token::Asterisk | 
                        Token::Slash | Token::Percent | Token::Caret | Token::TwoAsterisk => {
                        let operator = Self::token_to_operator(token)?;
                        let tree_op = Self::get_tree_element_operator(tree)?;
                        if tree_op.priority(&operator) {
                            self.search_add_point(tree.right_mut().unwrap())?;
                        } else {
                            Self::replace_and_add_left(tree, operator);
                            if !tree.left().unwrap().is_non_empty() {
                                return Err(format!("syntax error"))
                            }
                            self.index_plus();
                            self.while_next_token(tree)?;
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
            Token::TwoAsterisk => Ok(Operator::Pow),
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

    fn while_next_token(&mut self, tree: &mut BinaryTree<Element>) -> Result<bool, String> {
        while self.index < self.tokens.len() {
            if self.next_token(tree)? {
                return Ok(true)
            }
        }
        Ok(false)
    }

    fn next_token(&mut self, tree: &mut BinaryTree<Element>) -> Result<bool, String> {
        let token = self.get_next_token()?;
        match token {
            Token::NumString(s) => {
                let num = Self::string_to_num(s)?;
                self.add_num(tree, num)?;
                self.index_plus();
            },
            Token::Plus => return self.add_operator(tree, Operator::Plus),
            Token::Minus => return self.add_operator(tree, Operator::Minus),
            Token::Asterisk => return self.add_operator(tree, Operator::Mul),
            Token::Slash => return self.add_operator(tree, Operator::Div),
            Token::Percent => return self.add_operator(tree, Operator::Rem),
            Token::Caret => return self.add_operator(tree, Operator::Pow),
            Token::LParen => return self.add_paren(tree),
            Token::RParen => return Ok(true),
            Token::String(s) => {
                let string_box = s.clone();
                return self.add_variable(tree, string_box)
            },
            _ => return Err(format!("{:?}: wip", token))
        }
        Ok(false)
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
                    Element::Num(_) | Element::Dummy | Element::Variable(_)
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

    fn add_operator(&mut self, tree: &mut BinaryTree<Element>, operator: Operator) -> Result<bool, String> {
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
                    Element::Num(_) | Element::Variable(_) => {
                        Self::replace_and_add_left(tree, operator);
                        self.index_plus();
                    },
                    Element::Operator(tree_op) => {
                        if !tree.right().unwrap().is_non_empty() {
                            return Err(format!("syntax error"))
                        }
                        if tree_op.priority(&operator) {
                            return self.while_next_token(tree.right_mut().unwrap())
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

    fn add_paren(&mut self, tree: &mut BinaryTree<Element>) -> Result<bool, String> {
        let paren_tree = match tree {
            BinaryTree::Empty => {
                *tree = BinaryTree::from_element(Element::Operator(Operator::Paren));
                self.index_plus();
                self.root_tree_loop(tree.left_mut().unwrap())?;
                tree
            },
            BinaryTree::NonEmpty(node_box) => {
                match node_box.element {
                    Element::Num(_) | Element::Variable(_) => {
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
                self.root_tree_loop(right_tree.left_mut().unwrap())?;
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
                    Element::Num(_) | Element::Variable(_) => {
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

    pub fn calculation(&self, tree: &mut BinaryTree<Element>, data_base: &DataBase) -> Result<Option<Num>, String> {
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
                    Element::Variable(v) => {
                        match data_base.get(v) {
                            None => return Ok(None),
                            Some(n) => {
                                *tree = BinaryTree::from_element(Element::Num(n.clone()));
                                return Ok(Some(n.clone()))
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
                    self.calculation(tree.left_mut().unwrap(), data_base)?;
                let right_value_option =
                    self.calculation(tree.right_mut().unwrap(), data_base)?;
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
                            _ => todo!("wip matrix pattern"),
                        }
                    },
                    (None, Some(right_value)) => {
                        Self::add_paren_to_negative_value(tree, &right_value, &op, true)?;
                        return Ok(None)
                    },
                    (Some(left_value), None) => {
                        Self::add_paren_to_negative_value(tree, &left_value, &op, false)?;
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

    fn add_paren_to_negative_value(tree: &mut BinaryTree<Element>, value: &Num, op: &Operator, right: bool) -> Result<(), String> {
        if value.is_negative() {
            match op {
                Operator::Minus | Operator::Plus => {
                    if right {
                        let num = Num::Float(-1.0);
                        let num = value.supported_mul(&num).unwrap();
                        *tree.right_mut().unwrap() = BinaryTree::from_element(Element::Num(num));
                        if let Operator::Minus = op {
                            tree.set_element(Element::Operator(Operator::Plus));
                        } else {
                            tree.set_element(Element::Operator(Operator::Minus));
                        }
                    }
                },
                Operator::Mul | Operator::Div | Operator::Rem | Operator::Pow | Operator::MatrixMul => {
                    let tmp_tree = if right {tree.right_mut().unwrap()} else {tree.left_mut().unwrap()};
                    *tmp_tree = BinaryTree::from_element_and_tree(
                        Element::Operator(Operator::Paren),
                        BinaryTree::from_element(Element::Num(value.clone())),
                        BinaryTree::from_element(Element::Operator(Operator::RParen)),
                    )
                }
                Operator::Paren | Operator::RParen => return Err(format!("syntax error"))
            }
        }
        Ok(())
    }

    pub fn print_tree(&self, tree: &BinaryTree<Element>) -> Result<String, String> {
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
                }
            },
        }
        Ok(())
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
        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree() {
            Ok(v) => v,
            Err(e) => return Err(format!("error parser: {}", e))
        };
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register(&name, Num::Float(2.0));
        let name = "y".to_string();
        data_base.register(&name, Num::Float(-2.0));
        match parser.calculation(&mut tree, &data_base) {
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
        let mut parser = Parser::new(vec);
        match parser.make_tree() {
            Ok(v) => {
                let tree = v;
                match parser.print_tree(&tree) {
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
        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree() {
            Ok(v) => v,
            Err(e) => return format!("error parser: {}", e)
        };
        let mut data_base = DataBase::new();
        let name = "x".to_string();
        data_base.register(&name, Num::Float(2.0));
        let name = "y".to_string();
        data_base.register(&name, Num::Float(-2.0));
        match parser.calculation(&mut tree, &data_base) {
            Ok(_) => {
                match parser.print_tree(&tree) {
                    Ok(s) => s,
                    Err(e) => format!("error print_tree: {}", e),
                }
            }
            Err(e) => format!("error calculation: {}", e)
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
    fn calculation_and_print_plus_minus() {
        let code = "a + y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a - 2"))
    }

    #[test]
    fn calculation_and_print_mul_minus() {
        let code = "a * y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("a * ( -2 )"))
    }

    #[test]
    fn calculation_and_print_minus_mul() {
        let code = "y ^ a".to_string();
        assert_eq!(calculation_and_print_test(code), format!("( -2 ) ^ a"))
    }

    #[test]
    fn calculation_and_print_axy() {
        let code = "- 1 + 2 (x + a) ^ 2 * 3 - 2y".to_string();
        assert_eq!(calculation_and_print_test(code), format!("-1 + 2 * ( 2 + a ) ^ 2 * 3 + 4"))
    }
}