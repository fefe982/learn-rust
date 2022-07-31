// https://leetcode.com/problems/maximum-width-of-binary-tree/
// 662. Maximum Width of Binary Tree
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
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
    use super::*;
    #[test]
    fn del_nodes() {
        assert_eq!(
            Solution::width_of_binary_tree(TreeNode::from_vec(&vec![1, 3, 2, 5, 3, -1, 9])),
            4
        );
        assert_eq!(
            Solution::width_of_binary_tree(TreeNode::from_vec(&vec![
                1, 3, 2, 5, -1, -1, 9, 6, -1, 7
            ])),
            7
        );
        assert_eq!(
            Solution::width_of_binary_tree(TreeNode::from_vec(&vec![1, 3, 2, 5])),
            2
        );
    }
}
