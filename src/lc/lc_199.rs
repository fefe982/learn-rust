// https://leetcode.com/problems/binary-tree-right-side-view/
// 199. Binary Tree Right Side View
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, res: &mut Vec<i32>) {
        if let Some(node) = root {
            if res.len() == depth {
                res.push(node.borrow().val);
            }
            Self::walk(node.borrow_mut().right.take(), depth + 1, res);
            Self::walk(node.borrow_mut().left.take(), depth + 1, res);
        }
    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Self::walk(root, 0, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_right_side_view() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::right_side_view(TreeNode::from_vec(vec![1, 2, 3, null, 5, null, 4])),
            [1, 3, 4]
        );
        assert_eq!(
            Solution::right_side_view(TreeNode::from_vec(vec![1, 2, 3, 4, null, null, null, 5])),
            [1, 3, 4, 5]
        );
        assert_eq!(Solution::right_side_view(TreeNode::from_vec(vec![1, null, 3])), [1, 3]);
        assert_eq!(
            Solution::right_side_view(TreeNode::from_vec(vec![])),
            vec![] as Vec<i32>
        );
    }
}
