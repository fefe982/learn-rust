// https://leetcode.cn/problems/insufficient-nodes-in-root-to-leaf-paths/
// 1080. Insufficient Nodes in Root to Leaf Paths
use super::binary_tree::TreeNode;
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sufficient_subset_helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
        sum: i32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, bool) {
        if let Some(n) = root {
            let n = n.borrow();
            let nsum = sum + n.val;
            let left = Self::sufficient_subset_helper(&n.left, limit, nsum);
            let right = Self::sufficient_subset_helper(&n.right, limit, nsum);
            match (left, right) {
                ((None, true), (None, true)) => {
                    if nsum >= limit {
                        (Some(Rc::new(RefCell::new(TreeNode::new(n.val)))), false)
                    } else {
                        (None, false)
                    }
                }
                ((None, _), (None, _)) => (None, false),
                ((l, _), (r, _)) => {
                    let mut nroot = TreeNode::new(n.val);
                    nroot.left = l;
                    nroot.right = r;
                    (Some(Rc::new(RefCell::new(nroot))), false)
                }
            }
        } else {
            (None, true)
        }
    }
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sufficient_subset_helper(&root, limit, 0).0
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
