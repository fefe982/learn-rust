use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub const NULL: i32 = i32::MIN;
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn from_vec(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.len() == 0 {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(vec[0]))));
        let mut que = VecDeque::new();
        que.push_back(root.clone());
        let mut idx = 1;
        while let Some(parent) = que.pop_front() {
            let parent_node = parent.clone().unwrap();
            if idx < vec.len() && vec[idx] != NULL {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(vec[idx]))));
                que.push_back(new_node.clone());
                parent_node.as_ref().borrow_mut().left = new_node.clone();
            }
            if idx + 1 < vec.len() && vec[idx + 1] != NULL {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(vec[idx + 1]))));
                que.push_back(new_node.clone());
                parent_node.as_ref().borrow_mut().right = new_node.clone();
            }
            idx += 2;
        }
        root
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn from_vec() {
        assert_eq!(
            TreeNode::from_vec(vec![2, NULL, 3, NULL, 4]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                })))
            })))
        );
    }
}
