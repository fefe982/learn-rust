// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
// 235. Lowest Common Ancestor of a Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut r = root;
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        loop {
            let node = r.unwrap();
            let node = node.borrow();
            if node.val == q || node.val == p {
                return Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
            }
            if node.val > p.max(q) {
                r = node.left.clone();
            } else if node.val < p.min(q) {
                r = node.right.clone();
            } else {
                return Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lowest_common_ancestor() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec(vec![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]),
                TreeNode::from_vec(vec![2]),
                TreeNode::from_vec(vec![8])
            ),
            TreeNode::from_vec(vec![6])
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec(vec![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]),
                TreeNode::from_vec(vec![2]),
                TreeNode::from_vec(vec![4])
            ),
            TreeNode::from_vec(vec![2])
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec(vec![2, 1]),
                TreeNode::from_vec(vec![2]),
                TreeNode::from_vec(vec![1])
            ),
            TreeNode::from_vec(vec![2])
        );
    }
}
