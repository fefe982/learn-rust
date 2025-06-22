// https://leetcode.com/problems/count-complete-tree-nodes/
// 222. Count Complete Tree Nodes
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn count(root: Option<Rc<RefCell<TreeNode>>>, lvl: i32, expect: i32) -> (i32, i32, i32) {
        if let Some(node) = root {
            let (l, hl, expl) = Self::count(node.borrow_mut().left.take(), lvl + 1, expect - 1);
            if hl > 0 || expl < expect - 1 {
                return (l * 2 + 1, hl, expl + 1);
            }
            let (r, hr, _) = Self::count(node.borrow_mut().right.take(), lvl + 1, expl);
            if hr > 0 || r < l {
                (r * 2 + 1, hr + l - r, expl + 1)
            } else {
                (r * 2 + 1, 0, expl + 1)
            }
        } else {
            (0, 0, 0)
        }
    }
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let cnt = Self::count(root, 0, 0);
        cnt.0 + cnt.1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_nodes() {
        assert_eq!(Solution::count_nodes(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6])), 6);
        assert_eq!(Solution::count_nodes(TreeNode::from_vec(vec![])), 0);
    }
}
