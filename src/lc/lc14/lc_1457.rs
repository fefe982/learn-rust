// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/
// 1457. Pseudo-Palindromic Paths in a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn count(root: &Option<Rc<RefCell<TreeNode>>>, c: i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let nc = c ^ (1 << node.borrow().val);
                match (&node.borrow().left, &node.borrow().right) {
                    (&None, &None) => {
                        if (nc & (nc - 1)) == 0 {
                            1
                        } else {
                            0
                        }
                    }
                    (left, right) => Self::count(left, nc) + Self::count(right, nc),
                }
            }
        }
    }
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count(&root, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_pseudo_palindromic_paths() {
        let null = NULL;
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_vec(vec![2, 3, 1, 3, 1, null, 1])),
            2
        );
        assert_eq!(Solution::pseudo_palindromic_paths(TreeNode::from_vec(vec![9])), 1);
    }
}
