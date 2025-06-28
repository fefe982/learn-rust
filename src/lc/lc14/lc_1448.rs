// https://leetcode.com/problems/count-good-nodes-in-binary-tree/
// 1448. Count Good Nodes in Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn gnode(root: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        let mut cnt = 0;
        if let Some(node) = root {
            let node = node.borrow();
            let mut m = max;
            if node.val >= max {
                cnt += 1;
                m = node.val;
            }
            cnt += Self::gnode(&node.left, m);
            cnt += Self::gnode(&node.right, m);
        }
        cnt
    }
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::gnode(&root, i32::MIN)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn good_nodes() {
        let null = NULL;
        assert_eq!(
            Solution::good_nodes(TreeNode::from_vec(vec![3, 1, 4, 3, null, 1, 5])),
            4
        );
        assert_eq!(
            Solution::good_nodes(TreeNode::from_vec(vec![3, 3, null, 4, 2])),
            3
        );
        assert_eq!(Solution::good_nodes(TreeNode::from_vec(vec![1])), 1);
    }
}
