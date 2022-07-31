// https://leetcode.com/problems/minimum-depth-of-binary-tree/
// 111. Minimum Depth of Binary Tree
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let None = root {
            return 0;
        }
        let mut level = 1;
        let mut nodes = vec![root.clone().unwrap()];
        loop {
            let mut deeper_nodes = Vec::new();
            for node in nodes {
                let ref_node = node.borrow();
                if let TreeNode {
                    left: None,
                    right: None,
                    ..
                } = ref_node.deref()
                {
                    return level;
                }
                if let Some(left) = &ref_node.left {
                    deeper_nodes.push(left.clone())
                }
                if let Some(right) = &ref_node.right {
                    deeper_nodes.push(right.clone())
                }
            }
            nodes = deeper_nodes;
            level += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_depth() {
        assert_eq!(
            Solution::min_depth(TreeNode::from_vec(&vec![3, 9, 20, -1, -1, 15, 7])),
            2
        );
        assert_eq!(
            Solution::min_depth(TreeNode::from_vec(&vec![2, -1, 3, -1, 4, -1, 5, -1, 6])),
            5
        );
    }
}
