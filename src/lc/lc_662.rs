// https://leetcode.com/problems/maximum-width-of-binary-tree/
// 662. Maximum Width of Binary Tree
use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cmp;
pub struct Solution;
impl Solution {
    fn traverse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        minimum: &mut Vec<i32>,
        maximum: &mut Vec<i32>,
        level: usize,
        val: i32,
    ) {
        if let Some(node) = root {
            if level >= minimum.len() {
                minimum.push(val);
                maximum.push(val);
            } else {
                maximum[level] = val;
            }
            Self::traverse(&node.borrow().left, minimum, maximum, level + 1, val * 2);
            Self::traverse(
                &node.borrow().right,
                minimum,
                maximum,
                level + 1,
                val * 2 + 1,
            );
        }
    }
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut minimum = Vec::new();
        let mut maximum = Vec::new();
        Self::traverse(&root, &mut minimum, &mut maximum, 0, 0);
        let mut m = -1;
        for (min, max) in minimum.iter().zip(maximum.iter()) {
            m = cmp::max(m, max - min + 1);
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    fn from_array(vec: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(vec[0]))));
        let mut que = VecDeque::new();
        que.push_back(root.clone());
        let mut idx = 1;
        while let Some(parent) = que.pop_front() {
            let parent_node = parent.clone().unwrap();
            if idx < vec.len() && vec[idx] >= 0 {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(vec[idx]))));
                que.push_back(new_node.clone());
                parent_node.as_ref().borrow_mut().left = new_node.clone();
            }
            if idx + 1 < vec.len() && vec[idx + 1] >= 0 {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(vec[idx + 1]))));
                que.push_back(new_node.clone());
                parent_node.as_ref().borrow_mut().right = new_node.clone();
            }
            idx += 2;
        }
        root
    }
    #[test]
    fn del_nodes() {
        assert_eq!(
            Solution::width_of_binary_tree(from_array(&vec![1, 3, 2, 5, 3, -1, 9])),
            4
        );
        assert_eq!(
            Solution::width_of_binary_tree(from_array(&vec![1, 3, 2, 5, -1, -1, 9, 6, -1, 7])),
            7
        );
        assert_eq!(
            Solution::width_of_binary_tree(from_array(&vec![1, 3, 2, 5])),
            2
        );
    }
}
