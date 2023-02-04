#[derive(Debug, PartialEq)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}


#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


impl<T> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree::Empty
    }

    pub fn from_element(element: T) -> BinaryTree<T> {
        BinaryTree::NonEmpty(Box::new(TreeNode {
            element,
            left: BinaryTree::new(),
            right: BinaryTree::new(),
        }))
    }

    // pub fn add_left_node_from_element(element: T) {

    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_element_int() {
        let tree = BinaryTree::from_element(1);
        assert_eq!(format!("{:?}", tree), "NonEmpty(TreeNode { element: 1, left: Empty, right: Empty })".to_string());
    }
}