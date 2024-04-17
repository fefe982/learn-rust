// https://leetcode.com/problems/smallest-string-starting-from-leaf/
// 988. Smallest String Starting From Leaf
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn build_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        tree: &mut Vec<usize>,
        vals: &mut Vec<i32>,
        min_leaf: &mut i32,
        min_leaves: &mut Vec<usize>,
        p: usize,
    ) {
        if let Some(node) = root {
            let n = tree.len();
            tree.push(p);
            vals.push(node.borrow().val);
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if node.borrow().val == *min_leaf {
                    min_leaves.push(n);
                } else if node.borrow().val < *min_leaf {
                    min_leaves.clear();
                    min_leaves.push(n);
                    *min_leaf = node.borrow().val;
                }
            }
            Self::build_tree(node.borrow().left.clone(), tree, vals, min_leaf, min_leaves, n);
            Self::build_tree(node.borrow().right.clone(), tree, vals, min_leaf, min_leaves, n);
        }
    }
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut tree = vec![];
        let mut vals = vec![];
        let mut min_leaves = vec![];
        Self::build_tree(root, &mut tree, &mut vals, &mut 30, &mut min_leaves, usize::MAX);
        let mut cur_node = min_leaves.clone();
        while min_leaves.len() > 1 {
            let mut next_node = vec![];
            let mut next_min_leaves = vec![];
            let mut min_node = 30;
            for (idx, node) in cur_node.into_iter().enumerate() {
                let nnode = tree[node];
                if nnode == usize::MAX {
                    next_min_leaves = vec![min_leaves[idx]];
                    next_node = vec![nnode];
                    break;
                }
                if vals[nnode] < min_node {
                    min_node = vals[nnode];
                    next_node = vec![nnode];
                    next_min_leaves = vec![min_leaves[idx]];
                } else if vals[nnode] == min_node {
                    next_node.push(nnode);
                    next_min_leaves.push(min_leaves[idx]);
                }
            }
            min_leaves = next_min_leaves;
            cur_node = next_node;
        }
        let mut res = "".to_string();
        let mut node = min_leaves[0];
        while node != usize::MAX {
            res.push((vals[node] as u8 + b'a') as char);
            node = tree[node];
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_smallest_from_leaf() {
        let null = NULL;
        assert_eq!(
            Solution::smallest_from_leaf(TreeNode::from_vec(vec![0, 1, 2, 3, 4, 3, 4])),
            "dba"
        );
        assert_eq!(
            Solution::smallest_from_leaf(TreeNode::from_vec(vec![25, 1, 3, 1, 3, 0, 2])),
            "adz"
        );
        assert_eq!(
            Solution::smallest_from_leaf(TreeNode::from_vec(vec![2, 2, 1, null, 1, 0, null, 0])),
            "abc"
        );
    }
}
