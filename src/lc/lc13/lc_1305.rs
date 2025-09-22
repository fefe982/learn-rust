// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
// 1305. All Elements in Two Binary Search Trees
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn collect(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::collect(node.left.clone(), vec);
            vec.push(node.val);
            Self::collect(node.right.clone(), vec);
        }
    }
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v1 = vec![];
        let mut v2 = vec![];
        Self::collect(root1, &mut v1);
        Self::collect(root2, &mut v2);
        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < v1.len() && j < v2.len() {
            if v1[i] < v2[j] {
                ans.push(v1[i]);
                i += 1;
            } else {
                ans.push(v2[j]);
                j += 1;
            }
        }
        while i < v1.len() {
            ans.push(v1[i]);
            i += 1;
        }
        while j < v2.len() {
            ans.push(v2[j]);
            j += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn get_all_elements() {
        let null = NULL;
        assert_eq!(
            Solution::get_all_elements(TreeNode::from_vec(vec![2, 1, 4]), TreeNode::from_vec(vec![1, 0, 3])),
            vec![0, 1, 1, 2, 3, 4]
        );
        assert_eq!(
            Solution::get_all_elements(TreeNode::from_vec(vec![1, null, 8]), TreeNode::from_vec(vec![8, 1])),
            vec![1, 1, 8, 8]
        )
    }
}
