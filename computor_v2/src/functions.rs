use crate::binary_tree::BinaryTree;
use crate::parser::Element;
use crate::num::Num;
use crate::operator::Operator;


pub fn make_builtin_func_box(func_name: String) -> Box<(BinaryTree<Element>, String)> {
    Box::new((BinaryTree::from_element_and_tree(
        Element::Func(Box::new(func_name)),
        BinaryTree::from_element_and_tree(
            Element::Operator(Operator::Paren),
            BinaryTree::from_element(Element::Variable(Box::new("x".to_string()))),
            BinaryTree::from_element(Element::Operator(Operator::RParen))),
        BinaryTree::from_element(Element::Operator(Operator::RParen))
    ), "x".to_string()))
}


pub fn builtin_func(func_name: String, num: &Num) -> Result<Num, String> {
    let n = if func_name == "exp" {
        exp(num)?
    } else if func_name == "sqrt" {
        sqrt(num)?
    } else if func_name == "abs" {
        abs(num)?
    } else if func_name == "sin" {
        sin(num)?
    } else if func_name == "cos" {
        cos(num)?
    } else if func_name == "tan" {
        tan(num)?
    } else {
        return Err(format!("error: unsupported {}", func_name))
    };
    n.checked_value()?;
    Ok(n)
}


fn exp(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => Ok(Num::Float(n.exp())),
        _ => Err(format!("error: unsupported non float exp"))
    }
}


fn sqrt(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => {
            if n.is_sign_positive() || *n == 0.0 {
                Ok(Num::Float(n.sqrt()))
            } else {
                Ok(Num::from_two_float(0.0, n.abs().sqrt()))
            }
        }
        _ => Err(format!("error: unsupported non float sqrt"))
    }
}


fn abs(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => Ok(Num::Float(n.abs())),
        _ => Err(format!("error: unsupported non float abs"))
    }
}


fn sin(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => Ok(Num::Float(n.sin())),
        _ => Err(format!("error: unsupported non float sin"))
    }
}


fn cos(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => Ok(Num::Float(n.cos())),
        _ => Err(format!("error: unsupported non float cos"))
    }
}


fn tan(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => Ok(Num::Float(n.tan())),
        _ => Err(format!("error: unsupported non float tan"))
    }
}
