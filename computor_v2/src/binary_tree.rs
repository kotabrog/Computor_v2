#[derive(Debug, PartialEq)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}


#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    pub element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


impl<T> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree::Empty
    }

    pub fn from_node(node: TreeNode<T>) -> BinaryTree<T> {
        BinaryTree::NonEmpty(Box::new(node))
    }

    pub fn from_element(element: T) -> BinaryTree<T> {
        Self::from_node(TreeNode {
            element,
            left: BinaryTree::new(),
            right: BinaryTree::new(),
        })
    }

    pub fn is_non_empty(&self) -> bool {
        match self {
            BinaryTree::Empty => false,
            _ => true,
        }
    }

    pub fn left(&self) -> Option<&BinaryTree<T>> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(b) => Some(&b.left)
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut BinaryTree<T>> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(b) => Some(&mut b.left)
        }
    }

    pub fn right(&self) -> Option<&BinaryTree<T>> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(b) => Some(&b.right)
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut BinaryTree<T>> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(b) => Some(&mut b.right)
        }
    }

    pub fn add_left_node_from_tree(&mut self, tree: Self) -> BinaryTree<T> {
        let node_box = match self {
            Self::Empty => {
                *self = tree;
                return Self::Empty
            },
            Self::NonEmpty(b) => b,
        };
        std::mem::replace(&mut node_box.left, tree)
    }

    pub fn add_right_node_from_tree(&mut self, tree: Self) -> BinaryTree<T> {
        let node_box = match self {
            Self::Empty => {
                *self = tree;
                return Self::Empty
            },
            Self::NonEmpty(b) => b,
        };
        std::mem::replace(&mut node_box.right, tree)
    }

    pub fn add_left_node_from_node(&mut self, node: TreeNode<T>) -> BinaryTree<T> {
        self.add_left_node_from_tree(Self::from_node(node))
    }

    pub fn add_right_node_from_node(&mut self, node: TreeNode<T>) -> BinaryTree<T> {
        self.add_right_node_from_tree(Self::from_node(node))
    }

    pub fn add_left_node_from_element(&mut self, element: T) -> BinaryTree<T> {
        self.add_left_node_from_node(TreeNode {
            element,
            left: BinaryTree::new(),
            right: BinaryTree::new(),
        })
    }

    pub fn add_right_node_from_element(&mut self, element: T) -> BinaryTree<T> {
        self.add_right_node_from_node(TreeNode {
            element,
            left: BinaryTree::new(),
            right: BinaryTree::new(),
        })
    }

    pub fn delete_left_node(&mut self) -> BinaryTree<T> {
        let node_box = match self {
            Self::Empty => return Self::Empty,
            Self::NonEmpty(b) => b,
        };
        std::mem::replace(&mut node_box.left, BinaryTree::Empty)
    }

    pub fn delete_right_node(&mut self) -> BinaryTree<T> {
        let node_box = match self {
            Self::Empty => return Self::Empty,
            Self::NonEmpty(b) => b,
        };
        std::mem::replace(&mut node_box.right, BinaryTree::Empty)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_element_int() {
        let tree = BinaryTree::from_element(1);
        assert_eq!(format!("{:?}", tree), "NonEmpty(TreeNode { element: 1, left: Empty, right: Empty })".to_string());
    }

    #[test]
    fn is_non_empty_normal() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(!tree.is_non_empty());
        let tree = BinaryTree::from_element(1);
        assert!(tree.is_non_empty());
    }

    #[test]
    fn left_mut_normal() {
        let mut tree = BinaryTree::new();
        let tmp = tree.left_mut();
        assert_eq!(tmp, None);
        tree = BinaryTree::from_element("1".to_string());
        let tmp = tree.left_mut();
        assert_eq!(tmp, Some(&mut BinaryTree::Empty));
        tree.add_left_node_from_element("2".to_string());
        let tmp = tree.left_mut();
        assert_eq!(tmp, Some(&mut BinaryTree::from_element("2".to_string())));
        match tmp {
            Some(tree) => {
                tree.add_left_node_from_element("3".to_string());
            },
            _ => {}
        }
        let tmp = match tree.left_mut() {
            Some(tree) => {
                tree.left_mut()
            },
            None => None
        };
        assert_eq!(tmp, Some(&mut BinaryTree::from_element("3".to_string())));
    }

    #[test]
    fn right_normal() {
        let mut tree = BinaryTree::new();
        let tmp = tree.right_mut();
        assert_eq!(tmp, None);
        tree = BinaryTree::from_element("1".to_string());
        let tmp = tree.right_mut();
        assert_eq!(tmp, Some(&mut BinaryTree::Empty));
        tree.add_right_node_from_element("2".to_string());
        let tmp = tree.right_mut();
        assert_eq!(tmp, Some(&mut BinaryTree::from_element("2".to_string())));
        match tmp {
            Some(tree) => {
                tree.add_right_node_from_element("3".to_string());
            },
            _ => {}
        }
        let tmp = match tree.right_mut() {
            Some(tree) => {
                tree.right_mut()
            },
            None => None
        };
        assert_eq!(tmp, Some(&mut BinaryTree::from_element("3".to_string())));
    }

    #[test]
    fn add_left_node_from_element_normal() {
        let mut tree = BinaryTree::from_element("1".to_string());
        let tmp = tree.add_left_node_from_element("2".to_string());
        assert_eq!(tmp, BinaryTree::new());
        let tmp = tree.add_left_node_from_element("3".to_string());
        assert_eq!(tmp, BinaryTree::from_element("2".to_string()));
    }

    #[test]
    fn add_right_node_from_element_normal() {
        let mut tree = BinaryTree::from_element("1".to_string());
        let tmp = tree.add_right_node_from_element("2".to_string());
        assert_eq!(tmp, BinaryTree::new());
        let tmp = tree.add_right_node_from_element("3".to_string());
        assert_eq!(tmp, BinaryTree::from_element("2".to_string()));
    }

    #[test]
    fn delete_left_node_normal() {
        let mut tree = BinaryTree::from_element("1".to_string());
        tree.add_left_node_from_element("2".to_string());
        let tmp = tree.delete_left_node();
        assert_eq!(tmp, BinaryTree::from_element("2".to_string()));
        assert_eq!(tree, BinaryTree::from_element("1".to_string()));
    }

    #[test]
    fn delete_right_node_normal() {
        let mut tree = BinaryTree::from_element("1".to_string());
        tree.add_right_node_from_element("2".to_string());
        let tmp = tree.delete_right_node();
        assert_eq!(tmp, BinaryTree::from_element("2".to_string()));
        assert_eq!(tree, BinaryTree::from_element("1".to_string()));
    }
}
