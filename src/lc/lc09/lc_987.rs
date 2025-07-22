// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
// 987. Vertical Order Traversal of a Binary Tree
pub struct Solution;

use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        row: i32,
        col: i32,
        m: &mut std::collections::BTreeMap<i32, std::collections::BTreeMap<i32, std::collections::BinaryHeap<i32>>>,
    ) {
        if let Some(n) = node {
            let n = n.borrow();
            m.entry(col).or_default().entry(row).or_default().push(n.val);
            Self::dfs(n.left.clone(), row + 1, col - 1, m);
            Self::dfs(n.right.clone(), row + 1, col + 1, m);
        }
    }
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut m = std::collections::BTreeMap::new();
        Self::dfs(root, 0, 0, &mut m);
        m.into_iter()
            .map(|(_, row_map)| {
                row_map
                    .into_iter()
                    .map(|(_, v)| v.into_sorted_vec())
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    #[test]
    fn test_vertical_traversal() {
        let null = NULL;
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_vec(vec![
                0, 2, 1, 3, null, 5, 22, 9, 4, 12, 25, null, null, 13, 14, 8, 6, null, null, null, null, null, 27, 24,
                26, null, 17, 7, null, 28, null, null, null, null, null, 19, null, 11, 10, null, null, null, 23, 16,
                15, 20, 18, null, null, null, null, null, 21, null, null, 29
            ])),
            vec_vec![
                [13, 28],
                [9, 24, 27, 16],
                [3, 8, 14, 11, 19],
                [2, 4, 12, 7, 17, 26, 15, 20, 23],
                [0, 5, 6, 10, 21, 29],
                [1, 25, 18],
                [22]
            ]
        );
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])),
            vec_vec![[9], [3, 15], [20], [7]]
        );
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7])),
            vec_vec![[4], [2], [1, 5, 6], [3], [7]]
        );
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_vec(vec![1, 2, 3, 4, 6, 5, 7])),
            vec_vec![[4], [2], [1, 5, 6], [3], [7]]
        );
    }
}
