// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
// 865. Smallest Subtree with all the Deepest Nodes
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = root {
                let (ldepth, lnode) = dfs(&node.borrow().left);
                let (rdepth, rnode) = dfs(&node.borrow().right);
                if ldepth == rdepth {
                    (ldepth + 1, root.clone())
                } else if ldepth > rdepth {
                    (ldepth + 1, lnode)
                } else {
                    (rdepth + 1, rnode)
                }
            } else {
                (0, None)
            }
        }
        dfs(&root).1
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn subtree_with_all_deepest() {
        let null = NULL;
        assert_eq!(
            Solution::subtree_with_all_deepest(TreeNode::from_vec(vec![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4])),
            TreeNode::from_vec(vec![2, 7, 4])
        );
        assert_eq!(
            Solution::subtree_with_all_deepest(TreeNode::from_vec(vec![1])),
            TreeNode::from_vec(vec![1])
        );
        assert_eq!(
            Solution::subtree_with_all_deepest(TreeNode::from_vec(vec![0, 1, 3, null, 2])),
            TreeNode::from_vec(vec![2])
        );
    }
}
