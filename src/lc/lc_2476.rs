// https://leetcode.com/problems/closest-nodes-queries-in-a-binary-search-tree/
// 2476. Closest Nodes Queries in a Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn collect_tree(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::collect_tree(node.borrow_mut().left.take(), v);
            v.push(node.borrow().val);
            Self::collect_tree(node.borrow_mut().right.take(), v);
        }
    }
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut tree = vec![];
        Self::collect_tree(root, &mut tree);
        queries
            .into_iter()
            .map(|query| {
                let pos = tree.partition_point(|&x| x < query);
                if pos == tree.len() {
                    vec![tree[pos - 1], -1]
                } else if tree[pos] == query {
                    vec![query, query]
                } else {
                    vec![if pos == 0 { -1 } else { tree[pos - 1] }, tree[pos]]
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    #[test]
    fn test_closest_nodes() {
        let null = NULL;
        assert_eq!(
            Solution::closest_nodes(
                TreeNode::from_vec(vec![6, 2, 13, 1, 4, 9, 15, null, null, null, null, null, null, 14]),
                vec![2, 5, 16]
            ),
            vec_vec![[2, 2], [4, 6], [15, -1]]
        );
        assert_eq!(
            Solution::closest_nodes(TreeNode::from_vec(vec![4, null, 9]), vec![3]),
            vec_vec![[-1, 4]]
        );
    }
}
