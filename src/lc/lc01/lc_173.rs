// https://leetcode.com/problems/binary-search-tree-iterator/
// 173. Binary Search Tree Iterator
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn push(stack: &mut Vec<Rc<RefCell<TreeNode>>>, root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            stack.push(node.clone());
            BSTIterator::push(stack, &node.borrow().left);
        }
    }
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        BSTIterator::push(&mut stack, &root);
        Self { stack }
    }

    pub fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        BSTIterator::push(&mut self.stack, &node.borrow().right);
        return node.borrow().val;
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_binary_tree_tree_iterator() {
        let null = super::super::binary_tree::NULL;
        for test in [(
            [
                "BSTIterator",
                "next",
                "next",
                "hasNext",
                "next",
                "hasNext",
                "next",
                "hasNext",
                "next",
                "hasNext",
            ],
            [7, 3, 15, null, null, 9, 20],
            vec_any![null, 3, 7, true, 9, true, 15, true, 20, false],
        )] {
            let mut obj = BSTIterator::new(TreeNode::from_vec(test.1.to_vec()));
            for (cmd, expect) in test.0.into_iter().zip(test.2).skip(1) {
                match cmd {
                    "next" => {
                        assert_eq!(obj.next(), expect.as_i32())
                    }
                    "hasNext" => {
                        assert_eq!(obj.has_next(), expect.as_bool())
                    }
                    _ => {}
                }
            }
        }
    }
}
