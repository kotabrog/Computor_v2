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
    if func_name == "exp" {
        exp(num)
    } else {
        Err(format!("error: unsupported {}", func_name))
    }
}


fn exp(num: &Num) -> Result<Num, String> {
    match num {
        Num::Float(n) => Ok(Num::Float(n.exp())),
        _ => Err(format!("error: unsupported non float exp"))
    }
}
