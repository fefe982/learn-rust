// https://leetcode.com/problems/construct-string-from-binary-tree/
// 606. Construct String from Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            let mut res = node.borrow().val.to_string();
            let left = Self::tree2str(node.borrow_mut().left.take());
            let right = Self::tree2str(node.borrow_mut().right.take());
            if !right.is_empty() || !left.is_empty() {
                res.push('(');
                res.push_str(&left);
                res.push(')');
            }
            if !right.is_empty() {
                res.push('(');
                res.push_str(&right);
                res.push(')');
            }
            res
        } else {
            String::new()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_tree2str() {
        let null = NULL;
        assert_eq!(Solution::tree2str(TreeNode::from_vec(vec![1, 2, 3, 4])), "1(2(4))(3)");
        assert_eq!(
            Solution::tree2str(TreeNode::from_vec(vec![1, 2, 3, null, 4])),
            "1(2()(4))(3)"
        );
    }
}
