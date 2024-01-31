// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
// 1028. Recover a Tree From Preorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut t = vec![];
        let mut lvl = 0;
        for s in traversal.split('-') {
            if s.len() > 0 {
                let v = s.parse::<i32>().unwrap();
                let node = Rc::new(RefCell::new(TreeNode::new(v)));
                if lvl == 0 {
                    t.push(node);
                } else {
                    let p = t[lvl - 1].clone();
                    if p.borrow().left.is_none() {
                        p.borrow_mut().left = Some(node.clone());
                    } else {
                        p.borrow_mut().right = Some(node.clone());
                    }
                    if lvl >= t.len() {
                        t.push(node);
                    } else {
                        t[lvl] = node;
                    }
                }
                lvl = 1;
            } else {
                lvl += 1;
            }
        }
        Some(t[0].clone())
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_recover_from_preorder() {
        let null = NULL;
        assert_eq!(
            Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string()),
            TreeNode::from_vec(vec![1, 2, 5, 3, 4, 6, 7])
        );
        assert_eq!(
            Solution::recover_from_preorder("1-2--3---4-5--6---7".to_string()),
            TreeNode::from_vec(vec![1, 2, 5, 3, null, 6, null, 4, null, 7])
        );
        assert_eq!(
            Solution::recover_from_preorder("1-401--349---90--88".to_string()),
            TreeNode::from_vec(vec![1, 401, null, 349, 88, 90])
        );
    }
}
