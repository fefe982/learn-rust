// https://leetcode.com/problems/cousins-in-binary-tree/
// 993. Cousins in Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, x: i32, depth: i32, parent: i32) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val == x {
                return (depth, parent);
            }
            let (d, p) = Self::find_node(&node.left, x, depth + 1, node.val);
            if d != -1 {
                return (d, p);
            }
            let (d, p) = Self::find_node(&node.right, x, depth + 1, node.val);
            if d != -1 {
                return (d, p);
            }
        }
        (-1, -1)
    }
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let (dx, px) = Self::find_node(&root, x, 0, -1);
        let (dy, py) = Self::find_node(&root, y, 0, -1);
        dx == dy && px != py
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_is_cousins() {
        let null = NULL;
        assert_eq!(Solution::is_cousins(TreeNode::from_vec(vec![1, 2, 3, 4]), 4, 3), false);
        assert_eq!(
            Solution::is_cousins(TreeNode::from_vec(vec![1, 2, 3, null, 4, null, 5]), 5, 4),
            true
        );
        assert_eq!(
            Solution::is_cousins(TreeNode::from_vec(vec![1, 2, 3, null, 4]), 2, 3),
            false
        );
    }
}
