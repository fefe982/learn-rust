// https://leetcode.cn/problems/hSRGyL/
// LCP 26. 导航装置
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn cnt(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, bool) {
        if let Some(node) = root {
            let (l, r) = (Solution::cnt(&node.borrow().left), Solution::cnt(&node.borrow().right));
            if !l.2 && !r.2 {
                return (0, 1, true);
            }
            if l.2 && r.2 {
                return (
                    if l.0 == 0 && r.0 == 0 { 1 } else { l.0 + r.0 },
                    if l.0 > 0 && r.0 > 0 { 0 } else { 1 },
                    true,
                );
            }
            let c = if l.2 { l } else { r };
            return (c.0, c.1, true);
        } else {
            (0, 1, false)
        }
    }
    pub fn navigation(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let l = Solution::cnt(&root.as_ref().unwrap().borrow().left);
        let r = Solution::cnt(&root.as_ref().unwrap().borrow().right);
        if l.0 > 0 && r.0 > 0 {
            l.0 + r.0
        } else if l.0 == 0 {
            r.0 + r.1
        } else {
            l.0 + l.1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn navigation() {
        let null = NULL;
        assert_eq!(Solution::navigation(TreeNode::from_vec(vec![1, 2, null, 3, 4])), 2);
        assert_eq!(Solution::navigation(TreeNode::from_vec(vec![1, 2, 3, 4])), 1);
    }
}
