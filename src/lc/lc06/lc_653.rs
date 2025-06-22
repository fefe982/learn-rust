// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/
// 653. Two Sum IV - Input is a BST
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct BSTIterator {
    stack_l: Vec<Rc<RefCell<TreeNode>>>,
    stack_r: Vec<Rc<RefCell<TreeNode>>>,
}
impl BSTIterator {
    fn push(stack: &mut Vec<Rc<RefCell<TreeNode>>>, root: &Option<Rc<RefCell<TreeNode>>>, isleft: bool) {
        if let Some(node) = root {
            stack.push(node.clone());
            let next = if isleft {
                &node.borrow().left
            } else {
                &node.borrow().right
            };
            BSTIterator::push(stack, next, isleft);
        }
    }
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack_l = vec![];
        let mut stack_r = vec![];
        BSTIterator::push(&mut stack_l, &root, true);
        BSTIterator::push(&mut stack_r, &root, false);
        Self { stack_l, stack_r }
    }

    pub fn next(&mut self, isleft: bool) -> i32 {
        let stk = if isleft { &mut self.stack_l } else { &mut self.stack_r };
        let node = stk.pop().unwrap();
        let next = if isleft {
            &node.borrow().right
        } else {
            &node.borrow().left
        };
        BSTIterator::push(stk, next, isleft);
        return node.borrow().val;
    }

    pub fn has_next(&self, isleft: bool) -> bool {
        if isleft {
            !self.stack_l.is_empty()
        } else {
            !self.stack_r.is_empty()
        }
    }
}
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut iter = BSTIterator::new(root);
        let mut val_l = iter.next(true);
        let mut val_r = iter.next(false);
        while val_l < val_r {
            while val_l < val_r && val_l + val_r < k {
                val_l = iter.next(true);
            }
            if val_l == val_r {
                return false;
            }
            if val_l + val_r == k {
                return true;
            }
            while val_l < val_r && val_l + val_r > k {
                val_r = iter.next(false);
            }
            if val_l == val_r {
                return false;
            }
            if val_l + val_r == k {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn find_target() {
        let null = NULL;
        assert_eq!(Solution::find_target(TreeNode::from_vec(vec![2, null, 3]), 6), false);
        assert_eq!(
            Solution::find_target(TreeNode::from_vec(vec![5, 3, 6, 2, 4, null, 7]), 9),
            true
        );
        assert_eq!(
            Solution::find_target(TreeNode::from_vec(vec![5, 3, 6, 2, 4, null, 7]), 28),
            false
        );
    }
}
