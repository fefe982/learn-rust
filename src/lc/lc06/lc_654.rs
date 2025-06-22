// https://leetcode.com/problems/maximum-binary-tree/
// 654. Maximum Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn merge_stack(stack: &mut Vec<Rc<RefCell<TreeNode>>>, val: i32) {
        let mut sub = None;
        while let Some(node) = stack.pop() {
            if node.borrow().val < val {
                node.borrow_mut().right = sub;
                sub = Some(node);
            } else {
                stack.push(node);
                break;
            }
        }
        if let Some(node) = sub {
            stack.push(node);
        }
    }
    fn add_node(stack: &mut Vec<Rc<RefCell<TreeNode>>>, val: i32) {
        Self::merge_stack(stack, val);
        let n_node = Rc::new(RefCell::new(TreeNode::new(val)));
        if let Some(node) = stack.pop() {
            let root_val = node.borrow().val;
            if root_val < val {
                n_node.borrow_mut().left = Some(node);
            } else {
                stack.push(node);
            }
        }
        stack.push(n_node);
    }
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        for val in nums {
            Self::add_node(&mut stack, val);
        }
        Self::merge_stack(&mut stack, i32::MAX);
        stack.pop()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn construct_maximum_binary_tree() {
        let null = NULL;
        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
            TreeNode::from_vec(vec![6, 3, 5, null, 2, 0, null, null, 1])
        );
        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1]),
            TreeNode::from_vec(vec![3, null, 2, null, 1])
        );
    }
}
