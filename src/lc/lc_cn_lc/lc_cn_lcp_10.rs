// https://leetcode.cn/problems/er-cha-shu-ren-wu-diao-du/
// LCP 10. 二叉树任务调度
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (f64, f64) {
        if let Some(node) = root {
            let (l, r) = (Solution::dfs(&node.borrow().left), Solution::dfs(&node.borrow().right));
            let val = node.borrow().val as f64;
            (val + l.0 + r.0, l.1.max(r.1).max((l.0 + r.0) / 2.0) + val)
        } else {
            (0.0, 0.0)
        }
    }
    pub fn minimal_exec_time(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        Solution::dfs(&root).1
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn minimal_exec_time() {
        let null = NULL;
        assert_eq!(Solution::minimal_exec_time(TreeNode::from_vec(vec![47, 74, 31])), 121.0);
        assert_eq!(
            Solution::minimal_exec_time(TreeNode::from_vec(vec![15, 21, null, 24, null, 27, 26])),
            87.0
        );
        assert_eq!(
            Solution::minimal_exec_time(TreeNode::from_vec(vec![1, 3, 2, null, null, 4, 4])),
            7.5
        );
    }
}
