use crate::num::Num;
use crate::binary_tree::BinaryTree;
use crate::lexer::Token;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Rem,
    MatrixMul,
    Pow,
}

impl Operator {
    /// rhs has higher priority than self
    pub fn priority(&self, rhs: &Self) -> bool {
        match *self {
            Self::Plus | Self::Minus => {
                match rhs {
                    Self::Mul | Self::Div | Self::Rem | Self::MatrixMul | Self::Pow => true,
                    _ => false
                }
            },
            Self::Mul | Self::Div | Self::Rem | Self::MatrixMul => {
                match rhs {
                    Self::Pow => true,
                    _ => false
                }
            },
            _ => false
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Operator(Operator),
    Num(Num),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn make_tree(&mut self) -> Result<BinaryTree<Element>, String> {
        let mut tree = BinaryTree::new();
        while self.index < self.tokens.len() {
            match self.while_next_token(&mut tree)? {
                None => {},
                Some(element) => {
                    match element {
                        Element::Operator(next_op) => {
                            // self.add_operator(&mut tree, next_op)?
                            // let tmp_tree
                            //     = std::mem::replace(&mut tree,
                            //         BinaryTree::from_element(Element::Operator(next_op)));
                            // tree.add_left_node_from_tree(tmp_tree);
                        },
                        _ => return Err(format!("syntax error")),
                    }
                }
            }
        }
        Ok(tree)
    }

    fn while_next_token(&mut self, tree: &mut BinaryTree<Element>) -> Result<Option<Element>, String> {
        while self.index < self.tokens.len() {
            match self.next_token(tree)? {
                None => {},
                Some(element) => {
                    match element {
                        Element::Operator(next_op) => {
                            match tree {
                                BinaryTree::Empty => return Err(format!("syntax error")),
                                BinaryTree::NonEmpty(node) => {
                                    match node.element {
                                        Element::Operator(op) => {
                                            if next_op.priority(&op) {
                                                return Ok(Some(element))
                                            } else {
                                                let tmp_tree
                                                    = std::mem::replace(tree,
                                                        BinaryTree::from_element(Element::Operator(next_op)));
                                                tree.add_left_node_from_tree(tmp_tree);
                                                continue;
                                            }
                                        },
                                        _ => return Err(format!("syntax error")),
                                    }
                                }
                            }
                        },
                        _ => return Err(format!("syntax error"))
                    }
                }
            };
        }
        Ok(None)
    }

    fn get_next_token(&mut self) -> Result<&Token, String> {
        match self.tokens.get(self.index) {
            Some(v) => {
                self.index += 1;
                Ok(v)
            },
            None => Err(format!("syntax error")),
        }
    }

    fn next_token(&mut self, tree: &mut BinaryTree<Element>) -> Result<Option<Element>, String> {
        let token = self.get_next_token()?;
        let next_element = match token {
            Token::NumString(s) => {
                let num = Num::from_string_to_float(s)?;
                num.checked_value()?;
                self.add_num(tree, num)?
            },
            Token::Plus => self.add_operator(tree, Operator::Plus)?,
            Token::Minus => self.add_operator(tree, Operator::Minus)?,
            Token::Asterisk => self.add_operator(tree, Operator::Mul)?,
            Token::Slash => self.add_operator(tree, Operator::Div)?,
            Token::Percent => self.add_operator(tree, Operator::Rem)?,
            Token::Caret => self.add_operator(tree, Operator::Pow)?,
            _ => return Err(format!("{:?}: wip", token))
        };
        Ok(next_element)
    }

    fn add_num(&mut self, tree: &mut BinaryTree<Element>, num: Num) -> Result<Option<Element>, String> {
        let next_tree = match tree.left_mut() {
            None => tree,
            Some(l) => {
                match l {
                    BinaryTree::Empty => l,
                    BinaryTree::NonEmpty(_) => {
                        let r = tree.right_mut().unwrap();
                        if r.is_non_empty() {
                            return Err(format!("{:?}: syntax error", num))
                        }
                        r
                    },
                }
            },
        };
        *next_tree = BinaryTree::from_element(Element::Num(num));
        Ok(None)
    }

    fn add_operator(&mut self, tree: &mut BinaryTree<Element>, operator: Operator) -> Result<Option<Element>, String> {
        match tree {
            BinaryTree::Empty => {
                // *tree = BinaryTree::from_element(Element::Operator(operator));
                // wip: 単項演算子
                return Err(format!("{:?}: syntax error", operator))
            },
            BinaryTree::NonEmpty(e) => {
                match &e.element {
                    Element::Num(_) => {
                        let tmp_tree
                            = std::mem::replace(tree,
                                 BinaryTree::from_element(Element::Operator(operator)));
                        tree.add_left_node_from_tree(tmp_tree);
                        return Ok(None)
                    },
                    Element::Operator(op) => {
                        if op.priority(&operator) {
                            match tree.right() {
                                None => return Err(format!("{:?}: syntax error", operator)),
                                Some(_) => {
                                    self.add_operator(tree.right_mut().unwrap(), operator)?;
                                    Ok(self.while_next_token(tree.right_mut().unwrap())?)
                                }
                            }
                        } else {
                            Ok(Some(Element::Operator(operator)))
                            // let tmp_tree
                            //     = std::mem::replace(tree,
                            //         BinaryTree::from_element(Element::Operator(operator)));
                            // tree.add_left_node_from_tree(tmp_tree);
                            // return Ok(())
                        }
                    }
                }
            }
        }
    }

    pub fn calculation(&self, tree: &BinaryTree<Element>) -> Result<Num, String> {
        match tree {
            BinaryTree::Empty => return Err(format!("syntax error")),
            BinaryTree::NonEmpty(node_box) => {
                match &node_box.element {
                    Element::Operator(op) => {
                        let left_tree = tree.left().unwrap();
                        let right_tree = tree.right().unwrap();
                        let value = match (left_tree, right_tree) {
                            (BinaryTree::NonEmpty(l), BinaryTree::NonEmpty(r)) => {
                                let left_value = match &l.element {
                                    Element::Num(ln) => ln.clone(),
                                    Element::Operator(_) => self.calculation(left_tree)?,
                                };
                                let right_value = match &r.element {
                                    Element::Num(rn) => rn.clone(),
                                    Element::Operator(_) => self.calculation(right_tree)?,
                                };
                                match op {
                                    Operator::Plus => left_value.supported_add(&right_value)?,
                                    Operator::Minus => left_value.supported_sub(&right_value)?,
                                    Operator::Mul => left_value.supported_mul(&right_value)?,
                                    Operator::Div => left_value.supported_div(&right_value)?,
                                    Operator::Rem => left_value.supported_rem(&right_value)?,
                                    Operator::Pow => left_value.supported_pow(&right_value)?,
                                    _ => todo!("wip matrix pattern"),
                                }
                            },
                            _ => return Err(format!("syntax error")),
                        };
                        value.checked_value()?;
                        return Ok(value)
                    },
                    Element::Num(n) => return Ok(n.clone())
                }
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
        let mut parser = Parser::new(vec);
        let tree = match parser.make_tree() {
            Ok(v) => v,
            Err(e) => return Err(format!("error parser: {}", e))
        };
        match parser.calculation(&tree) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("error calculation: {}", e))
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
    fn calculation_long() {
        let code = "1 + 2 * 3 - 8 / 2 % 3 + 2 ^ 3 * 2".to_string();
        assert_eq!(calculation_test(code), Ok(Num::Float(22.0)))
    }
}