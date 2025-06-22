// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
// 236. Lowest Common Ancestor of a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> (bool, bool, i32) {
        if let Some(node) = root {
            let (left_p, left_q, val) = Self::find(&node.borrow().left, p, q);
            if left_p && left_q {
                return (left_p, left_q, val);
            }
            let (right_p, right_q, val) = Self::find(&node.borrow().right, p, q);
            if right_p && right_q {
                return (right_p, right_q, val);
            }
            return (
                left_p || right_p || node.borrow().val == p,
                left_q || right_q || node.borrow().val == q,
                node.borrow().val,
            );
        } else {
            (false, false, 0)
        }
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(
            Self::find(&root, p.unwrap().borrow().val, q.unwrap().borrow().val).2,
        ))))
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_lowest_common_ancestor() {
        let null = NULL;
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec(vec![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]),
                TreeNode::from_vec(vec![5]),
                TreeNode::from_vec(vec![1])
            ),
            TreeNode::from_vec(vec![3])
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec(vec![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]),
                TreeNode::from_vec(vec![5]),
                TreeNode::from_vec(vec![4])
            ),
            TreeNode::from_vec(vec![5])
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec(vec![1, 2]),
                TreeNode::from_vec(vec![1]),
                TreeNode::from_vec(vec![2])
            ),
            TreeNode::from_vec(vec![1])
        );
    }
}
