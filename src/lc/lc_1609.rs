// https://leetcode.com/problems/even-odd-tree/
// 1609. Even Odd Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn check_tree(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, last: &mut Vec<i32>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            if level % 2 == 0 {
                if val % 2 == 0 || (last.len() > level && last[level] >= val) {
                    return false;
                }
            } else {
                if val % 2 == 1 || (last.len() > level && last[level] <= val) {
                    return false;
                }
            }
            if last.len() == level {
                last.push(val);
            } else {
                last[level] = val;
            }
            if !Self::check_tree(&node.left, level + 1, last) || !Self::check_tree(&node.right, level + 1, last) {
                return false;
            }
        }
        true
    }
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_tree(&root, 0, &mut Vec::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_even_odd_tree() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::is_even_odd_tree(TreeNode::from_vec(vec![
                1, 10, 4, 3, null, 7, 9, 12, 8, 6, null, null, 2
            ])),
            true
        );
        assert_eq!(
            Solution::is_even_odd_tree(TreeNode::from_vec(vec![5, 4, 2, 3, 3, 7])),
            false
        );
        assert_eq!(
            Solution::is_even_odd_tree(TreeNode::from_vec(vec![5, 9, 1, 3, 5, 7])),
            false
        );
    }
}
