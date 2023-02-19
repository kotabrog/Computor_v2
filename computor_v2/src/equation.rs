use std::collections::HashMap;

use crate::binary_tree::BinaryTree;
use crate::parser::Element;
use crate::operator::Operator;
use crate::num::Num;


const MAX_TERMS: usize = 1000;


#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub degree: i64,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Equation {
    pub expr: Vec<Term>,
    pub degree: i64,
    pub variable: Option<String>,
}


impl Term {
    pub fn from_degree(degree: i64) -> Term {
        Term { coefficient: 1.0, degree }
    }

    pub fn from_coefficient(coefficient: f64) -> Term {
        Term { coefficient, degree: 0 }
    }
}


impl Equation {
    pub fn new() -> Equation {
        Equation { expr: vec![Term {coefficient: 0.0, degree: 0 }], degree: 0, variable: None }
    }

    pub fn make_equation(&mut self, lhs: &BinaryTree<Element>, rhs: &BinaryTree<Element>) -> Result<(), String> {
        let mut variable = None;
        self.set_one_tree(lhs, false, &mut variable)?;
        self.set_one_tree(rhs, true, &mut variable)?;
        println!("{:?}", self.expr);
        self.expr = Self::sort_expr(&self.expr)?;
        if self.expr.is_empty() {
            self.expr.push(Term { coefficient: 0.0, degree: 0 })
        }
        self.degree = self.expr.last().unwrap().degree;
        self.variable = variable;
        Ok(())
    }

    fn set_one_tree(&mut self, tree: &BinaryTree<Element>, right_side: bool, variable: &mut Option<String>) -> Result<(), String> {
        match tree {
            BinaryTree::Empty => return Err(format!("syntax error")),
            BinaryTree::NonEmpty(node_box) => {
                match &node_box.element {
                    Element::Dummy | Element::Func(_) => return Err(format!("syntax error")),
                    Element::Variable(_) | Element::Num(_) => self.set_one_terms(tree, right_side, variable)?,
                    Element::Operator(op) => {
                        match op {
                            Operator::Plus | Operator::Minus => {
                                self.set_one_tree(tree.left().unwrap(), right_side, variable)?;
                                self.set_one_tree(tree.right().unwrap(), right_side, variable)?;
                            },
                            _ => self.set_one_terms(tree, right_side, variable)?,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn set_one_terms(&mut self, tree: &BinaryTree<Element>, right_side: bool, variable: &mut Option<String>) -> Result<(), String> {
        let expr = self.set_one_term(tree, variable)?;
        for mut term in expr {
            if right_side {
                term.coefficient *= -1.0;
            }
            self.expr.push(term);
        }
        Ok(())
    }

    fn set_one_term(&mut self, tree: &BinaryTree<Element>, variable: &mut Option<String>) -> Result<Vec<Term>, String> {
        match tree {
            BinaryTree::Empty => return Err(format!("syntax error")),
            BinaryTree::NonEmpty(node_box) => {
                match &node_box.element {
                    Element::Func(_) => return Err(format!("syntax error")),
                    Element::Num(n)
                        => Ok(vec![Term::from_coefficient(Self::check_and_get_num_float(n)?)]),
                    Element::Dummy => Ok(vec![Term::from_coefficient(0.0)]),
                    Element::Variable(v) => {
                        Self::check_variable(variable, &v)?;
                        Ok(vec![Term::from_degree(1)])
                    },
                    Element::Operator(op) => {
                        match op {
                            Operator::Plus | Operator::Minus | Operator::RParen => Err(format!("syntax error")),
                            Operator::MatrixMul => Err(format!("Unsupported matrix product error.")),
                            Operator::Mul | Operator::Div | Operator::Rem => {
                                let expr_left = self.set_one_term(tree.left().unwrap(), variable)?;
                                let expr_right = self.set_one_term(tree.right().unwrap(), variable)?;
                                Ok(Self::operate_two_expr(&expr_left, &expr_right, op)?)
                            },
                            Operator::Pow => {
                                let expr_left = self.set_one_term(tree.left().unwrap(), variable)?;
                                let expr_right = self.set_one_term(tree.right().unwrap(), variable)?;
                                Ok(Self::operate_two_term_pow(&expr_left, &expr_right)?)
                            },
                            Operator::Paren => {
                                let mut equation = Equation::new();
                                equation.set_one_tree(tree.left().unwrap(), false, variable)?;
                                Ok(equation.expr)
                            }
                        }

                    }
                }
            }
        }
    }

    fn check_and_get_num_float(num: &Num) -> Result<f64, String> {
        match num {
            Num::Float(n) => Ok(*n),
            _ => return Err(format!("{}: syntax error: Not a real number.", num)),
        }
    }

    fn check_variable(variable: &mut Option<String>, new_variable: &String) -> Result<(), String> {
        match variable {
            Some(v) => {
                if v != new_variable {
                    return Err(format!("{}, {}: two variable error.", v, new_variable))
                }
            },
            None => *variable = Some(new_variable.clone()),
        }
        Ok(())
    }

    fn operate_two_expr(expr_left: &Vec<Term>, expr_right: &Vec<Term>, op: &Operator) -> Result<Vec<Term>, String> {
        let mut expr = Vec::new();
        match expr_left.len().checked_mul(expr_right.len()) {
            None => return Err(format!("too many terms error")),
            Some(n) => if n > MAX_TERMS {
                return Err(format!("too many terms error"))
            },
        }
        for left_term in expr_left {
            for right_term in expr_right {
                match op {
                    Operator::Mul => Self::operate_two_term_mul(left_term, right_term, &mut expr)?,
                    Operator::Div => Self::operate_two_term_div(left_term, right_term, &mut expr)?,
                    Operator::Rem => Self::operate_two_term_rem(left_term, right_term, &mut expr)?,
                    _ => return Err(format!("syntax error")),
                }
            }
        }
        if expr.is_empty() {
            expr.push(Term { coefficient: 0.0, degree: 0 });
        }
        Ok(expr)
    }

    fn operate_two_term_mul(left_term: &Term, right_term: &Term, expr: &mut Vec<Term>) -> Result<(), String> {
        let coefficient = left_term.coefficient * right_term.coefficient;
        if coefficient == 0.0 {
            return Ok(())
        }
        if !coefficient.is_finite() {
            return Err(format!("The calculation resulted in '{}'.", coefficient))
        }
        let degree = match left_term.degree.checked_add(right_term.degree) {
            None => return Err(format!("overflow error")),
            Some(n) => n,
        };
        expr.push(Term { coefficient, degree});
        Ok(())
    }

    fn operate_two_term_div(left_term: &Term, right_term: &Term, expr: &mut Vec<Term>) -> Result<(), String> {
        if right_term.degree > 0 {
            return Err(format!("error: cannot be divided by variable"))
        }
        let coefficient = left_term.coefficient / right_term.coefficient;
        if coefficient == 0.0 {
            return Ok(())
        }
        if !coefficient.is_finite() {
            return Err(format!("The calculation resulted in '{}'.", coefficient))
        }
        expr.push(Term { coefficient, degree: left_term.degree});
        Ok(())
    }

    fn operate_two_term_rem(left_term: &Term, right_term: &Term, expr: &mut Vec<Term>) -> Result<(), String> {
        if right_term.degree > 0 {
            return Err(format!("error: cannot be divided by variable"))
        }
        if left_term.degree > 0 {
            return Err(format!("error: variable remainders cannot be calculated"))
        }
        let coefficient = left_term.coefficient.rem_euclid(right_term.coefficient);
        if coefficient == 0.0 {
            return Ok(())
        }
        if !coefficient.is_finite() {
            return Err(format!("The calculation resulted in '{}'.", coefficient))
        }
        expr.push(Term { coefficient, degree: 0});
        Ok(())
    }

    fn operate_two_term_pow(expr_left: &Vec<Term>, expr_right: &Vec<Term>) -> Result<Vec<Term>, String> {
        let mut expr = Vec::new();
        let mut expr_right = Self::sort_expr(expr_right)?;
        if expr_right.len() == 0 {
            expr_right.push(Term { coefficient: 0.0, degree: 0 });
        }
        if expr_right.len() != 1
                || expr_right[0].degree != 0
                || !Self::is_int_value(expr_right[0].coefficient)
                || (expr_right[0].coefficient.is_sign_negative() && expr_right[0].coefficient != 0.0) {
            return Err(format!("error: only integers greater than or equal to 0 are allowed for exponents"))
        }
        let coefficient = expr_right[0].coefficient as u32;
        if coefficient == 0 {
            expr.push(Term { coefficient: 1.0, degree: 0 });
            return Ok(expr)
        }
        match expr_left.len().checked_pow(coefficient) {
            None => return Err(format!("too many terms error")),
            Some(n) => if n > MAX_TERMS {
                return Err(format!("too many terms error"))
            },
        }
        for term in expr_left {
            expr.push(term.clone());
        }
        for _ in 1..coefficient {
            let mut temp_expr = Vec::new();
            for left_term in &expr {
                for right_term in expr_left {
                    Self::operate_two_term_mul(left_term, right_term, &mut temp_expr)?;
                }
            }
            expr = temp_expr;
        }
        if expr.is_empty() {
            expr.push(Term { coefficient: 0.0, degree: 0 });
        }
        Ok(expr)
    }

    fn sort_expr(expr: &Vec<Term>) -> Result<Vec<Term>, String> {
        let mut terms = HashMap::new();
        for term in expr {
            let value = terms.entry(term.degree)
                .or_insert(Term { coefficient: 0.0, degree: term.degree});
            value.coefficient += term.coefficient;
            if !value.coefficient.is_finite() {
                return Err(format!("The calculation resulted in '{}'.", value.coefficient))
            }
            if value.coefficient == 0.0 {
                terms.remove(&term.degree);
            }
        }
        let mut vec: Vec<Term> = terms.into_values().collect();
        vec.sort_by(|a, b| a.degree.cmp(&b.degree));
        Ok(vec)
    }

    fn is_int_value(v: f64) -> bool {
        let int_v = v as i64;
        v - int_v as f64 == 0.0
    }

    fn to_string(&self) -> Result<String, String> {
        let mut string = String::new();
        for (i, term) in self.expr.iter().enumerate() {
            if i > 0 {
                if term.coefficient.is_sign_negative() {
                    string += format!("- ").as_str()
                } else {
                    string += format!("+ ").as_str()
                }
            } else {
                if term.coefficient.is_sign_negative() {
                    string += format!("-").as_str()
                }
            }
            if term.degree == 0 {
                string += format!("{} ", term.coefficient.abs()).as_str()
            } else if term.coefficient.abs() == 1.0 {
                string += format!("{}^{} ", self.get_variable_string()?, term.degree).as_str()
            } else {
                string += format!("{}{}^{} ", term.coefficient, self.get_variable_string()?, term.degree).as_str()
            }
        }
        string.pop();
        Ok(string)
    }

    fn get_variable_string(&self) -> Result<&String, String> {
        match &self.variable {
            Some(s) => Ok(s),
            None => Err(format!("error: get variable string")),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_base::DataBase;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn prepare_tree(code: String, data_base: &DataBase) -> Result<BinaryTree<Element>, String> {
        let mut lexer = Lexer::new(&code);
        let vec = match lexer.make_token_vec() {
            Ok(v) => v,
            Err(e) => return Err(format!("error lexer: {}", e))
        };

        let mut parser = Parser::new(vec);
        let mut tree = match parser.make_tree(&data_base) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parser: {}", e))
        };
        match parser.calculation(&mut tree, &data_base, None) {
            Ok(_) => {}
            Err(e) =>return  Err(format!("error calculation: {}", e))
        }
        Ok(tree)
    }

    fn make_equation_test(code1: String, code2: String) -> Result<Equation, String> {
        let mut data_base = DataBase::new();
        let name = "a".to_string();
        data_base.register_num(&name, Num::Float(2.0));
        let name = "b".to_string();
        data_base.register_num(&name, Num::Float(-2.0));

        let tree1 = prepare_tree(code1, &data_base)?;
        let tree2 = prepare_tree(code2, &data_base)?;

        let mut equation = Equation::new();
        equation.make_equation(&tree1, &tree2)?;
        Ok(equation)
    }

    #[test]
    fn make_equation_simple() -> Result<(), String> {
        let equation_result = make_equation_test(
            "1 + x + x^2".to_string(),
            "0".to_string(),
        );
        assert_eq!(match equation_result {
            Ok(equation) => match equation.to_string() {
                Ok(string) => string,
                Err(string) => string,
            },
            Err(string) => string,
        }, format!("1 + x^1 + x^2"));
        Ok(())
    }

    #[test]
    fn make_equation_zero() -> Result<(), String> {
        let equation_result = make_equation_test(
            "0".to_string(),
            "0".to_string(),
        );
        assert_eq!(match equation_result {
            Ok(equation) => match equation.to_string() {
                Ok(string) => string,
                Err(string) => string,
            },
            Err(string) => string,
        }, format!("0"));
        Ok(())
    }

    #[test]
    fn make_equation_const() -> Result<(), String> {
        let equation_result = make_equation_test(
            "2".to_string(),
            "0".to_string(),
        );
        assert_eq!(match equation_result {
            Ok(equation) => match equation.to_string() {
                Ok(string) => string,
                Err(string) => string,
            },
            Err(string) => string,
        }, format!("2"));
        Ok(())
    }

    #[test]
    fn make_equation_const_move() -> Result<(), String> {
        let equation_result = make_equation_test(
            "2".to_string(),
            "3".to_string(),
        );
        assert_eq!(match equation_result {
            Ok(equation) => match equation.to_string() {
                Ok(string) => string,
                Err(string) => string,
            },
            Err(string) => string,
        }, format!("-1"));
        Ok(())
    }

    #[test]
    fn make_equation_pow() -> Result<(), String> {
        let equation_result = make_equation_test(
            "x^2".to_string(),
            "0".to_string(),
        );
        assert_eq!(match equation_result {
            Ok(equation) => match equation.to_string() {
                Ok(string) => string,
                Err(string) => string,
            },
            Err(string) => string,
        }, format!("x^2"));
        Ok(())
    }
}