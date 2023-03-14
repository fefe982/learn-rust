// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
// 421. Maximum XOR of Two Numbers in an Array
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
struct TreeNode {
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            left: None,
            right: None,
        }
    }
}
impl Solution {
    fn insert_node(node: &mut Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        if let None = node {
            *node = Some(Rc::new(RefCell::new(TreeNode::new())));
        }
        node.as_mut().unwrap().clone()
    }
    fn adv(
        dir1: &Option<Rc<RefCell<TreeNode>>>,
        dir0: &Option<Rc<RefCell<TreeNode>>>,
        xor: &mut i32,
    ) -> Rc<RefCell<TreeNode>> {
        if let Some(n) = dir1 {
            *xor = *xor * 2 + 1;
            return n.clone();
        }
        if let Some(n) = dir0 {
            *xor = *xor * 2;
            return n.clone();
        }
        panic!();
    }
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let trie = Rc::new(RefCell::new(TreeNode::new()));
        for &n in nums.iter() {
            let mut cur = trie.clone();
            for i in (0..31).rev() {
                if n & (1 << i) != 0 {
                    cur = Self::insert_node(&mut cur.clone().borrow_mut().right);
                } else {
                    cur = Self::insert_node(&mut cur.clone().borrow_mut().left);
                }
            }
        }
        let mut max_xor = 0;
        for &n in nums.iter() {
            let mut cur = trie.clone();
            let mut xor = 0;
            for i in (0..31).rev() {
                if n & (1 << i) != 0 {
                    cur = Self::adv(
                        &cur.clone().borrow().left,
                        &cur.clone().borrow().right,
                        &mut xor,
                    );
                } else {
                    cur = Self::adv(
                        &cur.clone().borrow().right,
                        &cur.clone().borrow().left,
                        &mut xor,
                    );
                }
            }
            if xor > max_xor {
                max_xor = xor;
            }
        }
        max_xor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compress() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
    }
}
