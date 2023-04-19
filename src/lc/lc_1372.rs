// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree
// 1372. Longest ZigZag Path in a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn longest_zz(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(node) = root {
            let l = Self::longest_zz(&node.as_ref().borrow().left);
            let r = Self::longest_zz(&node.as_ref().borrow().right);
            let lm = l.2 + 1;
            let rm = r.1 + 1;
            let m = std::cmp::max(std::cmp::max(l.0, r.0), std::cmp::max(lm, rm));
            (m, lm, rm)
        } else {
            (0, 0, 0)
        }
    }
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        std::cmp::max(Self::longest_zz(&root).0 - 1, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_zig_zag() {
        assert_eq!(
            Solution::longest_zig_zag(TreeNode::from_vec(&vec![
                1, -1, 1, 1, 1, -1, -1, 1, 1, -1, 1, -1, -1, -1, 1, -1, 1
            ])),
            3
        );
        assert_eq!(
            Solution::longest_zig_zag(TreeNode::from_vec(&vec![
                1, 1, 1, -1, 1, -1, -1, 1, 1, -1, 1
            ])),
            4
        );
    }
}
