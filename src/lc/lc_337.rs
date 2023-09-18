// https://leetcode.com/problems/house-robber-iii/
// 337. House Robber III
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn rob_helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(node) => {
                let left = Solution::rob_helper(&node.borrow().left);
                let right = Solution::rob_helper(&node.borrow().right);
                let root = node.borrow().val + left.1 + right.1;
                (root.max(left.0 + right.0), left.0 + right.0)
            }
            None => (0, 0),
        }
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::rob_helper(&root).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_rob() {
        let null = NULL;
        assert_eq!(
            Solution::rob(TreeNode::from_vec(vec![3, 2, 3, null, 3, null, 1])),
            7
        );
        assert_eq!(
            Solution::rob(TreeNode::from_vec(vec![3, 4, 5, 1, 3, null, 1])),
            9
        );
    }
}
