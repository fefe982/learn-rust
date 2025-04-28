// https://leetcode.com/problems/find-mode-in-binary-search-tree/
// 501. Find Mode in Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find_mode_helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        cnt: &mut i32,
        last: &mut i32,
        mod_cnt: &mut i32,
        modes: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            Solution::find_mode_helper(&node.left, cnt, last, mod_cnt, modes);
            if node.val == *last {
                *cnt += 1;
            } else {
                *cnt = 1;
                *last = node.val;
            }
            if cnt == mod_cnt {
                modes.push(node.val);
            } else if cnt > mod_cnt {
                *modes = vec![node.val];
                *mod_cnt = *cnt;
            }
            Solution::find_mode_helper(&node.right, cnt, last, mod_cnt, modes);
        }
    }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        Self::find_mode_helper(&root, &mut 0, &mut -1, &mut 0, &mut ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_find_mode() {
        let null = NULL;
        assert_eq!(Solution::find_mode(TreeNode::from_vec(vec![1, null, 2, 2])), vec![2]);
        assert_eq!(Solution::find_mode(TreeNode::from_vec(vec![0])), vec![0]);
    }
}
