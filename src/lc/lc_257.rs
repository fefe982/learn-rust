// https://leetcode.com/problems/binary-tree-paths/
// 257. Binary Tree Paths

use super::binary_tree::TreeNode;
use std::cell::Ref;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    fn append_path(root: Ref<TreeNode>, prefix: &str) -> Vec<String> {
        let next_prefix = format!("{}->{}", prefix, root.val.to_string());
        Self::traverse_tree(root, next_prefix)
    }
    fn traverse_tree(root: Ref<TreeNode>, prefix: String) -> Vec<String> {
        if root.left == None && root.right == None {
            return vec![prefix];
        }
        let mut res = Vec::new();
        if let Some(ref left) = root.left {
            res.extend(Self::append_path(left.deref().borrow(), &prefix))
        }
        if let Some(ref right) = root.right {
            res.extend(Self::append_path(right.deref().borrow(), &prefix))
        }
        res
    }
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let root = root.as_ref().unwrap().deref().borrow();
        let prefix = root.val.to_string();
        Self::traverse_tree(root, prefix)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_tree_paths() {
        assert_eq!(
            Solution::binary_tree_paths(TreeNode::from_vec(&vec![1, 2, 3, -1, 5])),
            vec!["1->2->5".to_owned(), "1->3".to_owned()]
        );
        assert_eq!(
            Solution::binary_tree_paths(TreeNode::from_vec(&vec![1])),
            vec!["1".to_owned()]
        );
    }
}
