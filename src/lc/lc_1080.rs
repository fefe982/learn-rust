// https://leetcode.cn/problems/insufficient-nodes-in-root-to-leaf-paths/
// 1080. Insufficient Nodes in Root to Leaf Paths
use super::binary_tree::TreeNode;
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sufficient_subset_helper(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
        sum: i32,
    ) -> bool {
        if let Some(n) = root.take() {
            let nsum = sum + n.borrow().val;
            let ln = Self::sufficient_subset_helper(&mut n.borrow_mut().left, limit, nsum);
            let rn = Self::sufficient_subset_helper(&mut n.borrow_mut().right, limit, nsum);
            if ln && rn {
                if nsum >= limit {
                    *root = Some(n);
                }
            } else {
                match (&n.borrow().left, &n.borrow().right) {
                    (&None, &None) => (),
                    _ => *root = Some(n.clone()),
                }
            }
            false
        } else {
            true
        }
    }
    pub fn sufficient_subset(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sufficient_subset_helper(&mut root, limit, 0);
        root
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn sufficient_subset() {
        assert_eq!(
            Solution::sufficient_subset(
                TreeNode::from_vec(vec![
                    1, 2, 3, 4, -99, -99, 7, 8, 9, -99, -99, 12, 13, -99, 14
                ]),
                1
            ),
            TreeNode::from_vec(vec![1, 2, 3, 4, NULL, NULL, 7, 8, 9, NULL, 14])
        );
        assert_eq!(
            Solution::sufficient_subset(
                TreeNode::from_vec(vec![5, 4, 8, 11, NULL, 17, 4, 7, 1, NULL, NULL, 5, 3]),
                22
            ),
            TreeNode::from_vec(vec![5, 4, 8, 11, NULL, 17, 4, 7, NULL, NULL, NULL, 5])
        );
        assert_eq!(
            Solution::sufficient_subset(TreeNode::from_vec(vec![1, 2, -3, -5, NULL, 4, NULL]), -1),
            TreeNode::from_vec(vec![1, NULL, -3, 4])
        );
    }
}
