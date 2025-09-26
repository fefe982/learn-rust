// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/
// 1339. Maximum Product of Splitted Binary Tree
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum(root: &Option<Rc<RefCell<TreeNode>>>, subtree: &mut Vec<i32>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let l = Self::sum(&node.borrow().left, subtree);
                let r = Self::sum(&node.borrow().right, subtree);
                let s = node.borrow().val + l + r;
                subtree.push(s);
                s
            }
        }
    }
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = vec![];
        let s = Solution::sum(&root, &mut v);
        v.sort_unstable();
        let mut pos = v.partition_point(|&x| x * 2 < s);
        if pos == v.len() {
            pos -= 1;
        } else if pos > 0 && (s - v[pos] < v[pos - 1]) {
            pos -= 1;
        }
        ((s - v[pos]) as i64 * v[pos] as i64 % 1_000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn max_product() {
        let null = NULL;
        assert_eq!(Solution::max_product(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6])), 110);
        assert_eq!(
            Solution::max_product(TreeNode::from_vec(vec![1, null, 2, 3, 4, null, null, 5, 6])),
            90
        );
    }
}
