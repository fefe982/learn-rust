// https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
// 2415. Reverse Odd Levels of Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = vec![];
        nodes.push(root.clone().unwrap());
        let mut i = 0;
        loop {
            let node = nodes[i].clone();
            if node.borrow().left.is_some() {
                nodes.push(node.borrow().left.clone().unwrap());
                nodes.push(node.borrow().right.clone().unwrap());
            } else {
                break;
            }
            i += 1;
        }
        i = 1;
        loop {
            if 1 << i >= nodes.len() {
                break;
            }
            for j in (1 << i) - 1..(1 << i) - 1 + (1 << (i - 1)) {
                let val1 = nodes[j].borrow().val;
                let val2 = nodes[(3 << i) - 3 - j].borrow().val;
                nodes[j].borrow_mut().val = val2;
                nodes[(3 << i) - 3 - j].borrow_mut().val = val1;
            }
            i += 2;
        }
        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_odd_levels() {
        assert_eq!(
            Solution::reverse_odd_levels(TreeNode::from_vec(vec![2, 3, 5, 8, 13, 21, 34])),
            TreeNode::from_vec(vec![2, 5, 3, 8, 13, 21, 34])
        );
        assert_eq!(
            Solution::reverse_odd_levels(TreeNode::from_vec(vec![7, 13, 11])),
            TreeNode::from_vec(vec![7, 11, 13])
        );
        assert_eq!(
            Solution::reverse_odd_levels(TreeNode::from_vec(vec![0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2])),
            TreeNode::from_vec(vec![0, 2, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1])
        );
    }
}
