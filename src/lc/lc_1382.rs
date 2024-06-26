// https://leetcode.com/problems/balance-a-binary-search-tree/
// 1382. Balance a Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut v = Self::walk(node.borrow_mut().left.take());
            v.push(node.borrow().val);
            v.append(&mut Self::walk(node.borrow_mut().right.take()));
            v
        } else {
            vec![]
        }
    }
    fn construct(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() {
            return None;
        }
        let mid = arr.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: arr[mid],
            left: Self::construct(&arr[..mid]),
            right: Self::construct(&arr[mid + 1..]),
        })))
    }
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct(&Self::walk(root))
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    fn walk(root: &Option<Rc<RefCell<TreeNode>>>) -> (Vec<i32>, i32, bool) {
        if let Some(node) = root {
            let (mut v, ldep, lbalance) = walk(&node.borrow().left);
            v.push(node.borrow().val);
            let (mut rv, rdep, rbalance) = walk(&node.borrow().right);
            v.append(&mut rv);
            (v, ldep.max(rdep) + 1, lbalance && rbalance && (ldep - rdep).abs() <= 1)
        } else {
            (vec![], 0, true)
        }
    }
    fn check(arr: Vec<i32>) {
        let root = TreeNode::from_vec(arr);
        let (v, _, _) = walk(&root);
        let root = Solution::balance_bst(root);
        let (v2, _, balance) = walk(&root);
        assert_eq!(v, v2);
        assert!(balance);
    }
    #[test]
    fn test_balance_bst() {
        let null = NULL;
        check(vec![1, null, 2, null, 3, null, 4, null, null]);
        check(vec![2, 1, 3]);
    }
}
