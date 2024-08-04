// https://leetcode.com/problems/subtree-of-another-tree/
// 572. Subtree of Another Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut res = vec![node.borrow().val];
            res.extend(&Self::get_tree(node.borrow().left.clone()));
            res.extend(&Self::get_tree(node.borrow().right.clone()));
            res
        } else {
            vec![i32::MAX]
        }
    }
    fn build_next(v: &Vec<i32>) -> Vec<usize> {
        let mut res = vec![0; v.len()];
        let mut j = 0;
        for i in 1..v.len() {
            while j > 0 && v[i] != v[j] {
                j = res[j - 1];
            }
            if v[i] == v[j] {
                j += 1;
            }
            res[i] = j;
        }
        res
    }
    fn kmp(s: Vec<i32>, p: Vec<i32>) -> bool {
        let next = Self::build_next(&p);
        let mut j = 0;
        for i in 0..s.len() {
            while j > 0 && s[i] != p[j] {
                j = next[j - 1];
            }
            if s[i] == p[j] {
                j += 1;
            }
            if j == p.len() {
                return true;
            }
        }
        false
    }
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::kmp(Self::get_tree(root), Self::get_tree(sub_root))
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_is_subtree() {
        let null = NULL;
        assert_eq!(
            Solution::is_subtree(
                TreeNode::from_vec(vec![3, 4, 5, 1, 2]),
                TreeNode::from_vec(vec![4, 1, 2])
            ),
            true
        );
        assert_eq!(
            Solution::is_subtree(
                TreeNode::from_vec(vec![3, 4, 5, 1, 2, null, null, null, null, 0]),
                TreeNode::from_vec(vec![4, 1, 2])
            ),
            false
        );
    }
}
