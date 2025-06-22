// https://leetcode.com/problems/print-binary-tree/
// 655. Print Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn fill(root: &Option<Rc<RefCell<TreeNode>>>, h: usize, pos: usize, w: usize, grid: &mut Vec<Vec<String>>) {
        if let Some(node) = root {
            grid[h][pos] = node.borrow().val.to_string();
            Self::fill(&node.borrow().left, h + 1, pos - w, w / 2, grid);
            Self::fill(&node.borrow().right, h + 1, pos + w, w / 2, grid);
        }
    }
    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = Self::height(&node.borrow().left);
                let right = Self::height(&node.borrow().right);
                1 + std::cmp::max(left, right)
            }
        }
    }
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let h = Self::height(&root) as usize;
        let w = (1usize << h) - 1;
        let mut grid = vec![vec!["".to_string(); w]; h];
        Self::fill(&root, 0, (1usize << (h - 1)) - 1, (1usize << (h - 1)) / 2, &mut grid);
        grid
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    #[test]
    fn print_tree() {
        let null = NULL;
        assert_eq!(
            Solution::print_tree(TreeNode::from_vec(vec![1, 2])),
            vec_vec_str![["", "1", ""], ["2", "", ""]]
        );
        assert_eq!(
            Solution::print_tree(TreeNode::from_vec(vec![1, 2, 3, null, 4])),
            vec_vec_str![
                ["", "", "", "1", "", "", ""],
                ["", "2", "", "", "", "3", ""],
                ["", "", "4", "", "", "", ""]
            ]
        );
    }
}
