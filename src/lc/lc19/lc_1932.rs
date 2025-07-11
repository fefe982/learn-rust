// https://leetcode.com/problems/merge-bsts-to-create-single-bst/
// 1932. Merge BSTs to Create Single BST
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_tree(
        root: &Option<Rc<RefCell<TreeNode>>>,
        node_map: &std::collections::HashMap<i32, usize>,
        trees: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        bound: &mut Vec<(i32, i32)>,
        merged: &mut Vec<bool>,
        left_bound: i32,
        right_bound: i32,
    ) -> (bool, Option<usize>) {
        if let Some(node) = root {
            let val = node.borrow().val;
            if val <= left_bound || val >= right_bound {
                return (false, None);
            }
            if let Some(&tree_idx) = node_map.get(&val) {
                if bound[tree_idx].0 <= left_bound && bound[tree_idx].1 >= right_bound {
                    return (false, None);
                }
                if Self::merge_tree(tree_idx, node_map, trees, bound, merged, left_bound, right_bound) {
                    if bound[tree_idx].0 <= left_bound || bound[tree_idx].1 >= right_bound {
                        return (false, None);
                    }
                    (true, Some(tree_idx))
                } else {
                    (false, None)
                }
            } else {
                (true, None)
            }
        } else {
            (true, None)
        }
    }
    fn merge_tree(
        idx: usize,
        node_map: &std::collections::HashMap<i32, usize>,
        trees: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        bound: &mut Vec<(i32, i32)>,
        merged: &mut Vec<bool>,
        left_bound: i32,
        right_bound: i32,
    ) -> bool {
        if merged[idx] {
            return true;
        }
        merged[idx] = true;
        let root = trees[idx].take();
        let mut tree_bound_left = bound[idx].0;
        let mut tree_bound_right = bound[idx].1;
        if let Some(node) = root {
            {
                let mut node_ref = node.borrow_mut();
                let val = node_ref.val;
                if val <= left_bound || val >= right_bound {
                    return false;
                }
                let (left_ok, left_tree) =
                    Solution::get_tree(&mut node_ref.left, node_map, trees, bound, merged, left_bound, val);
                if !left_ok {
                    return false;
                }
                if let Some(left_tree_idx) = left_tree {
                    tree_bound_left = bound[left_tree_idx].0;
                    node_ref.left = trees[left_tree_idx].take();
                }
                let (right_ok, right_tree) =
                    Solution::get_tree(&mut node_ref.right, node_map, trees, bound, merged, val, right_bound);
                if !right_ok {
                    return false;
                }
                if let Some(right_tree_idx) = right_tree {
                    tree_bound_right = bound[right_tree_idx].1;
                    node_ref.right = trees[right_tree_idx].take();
                }
            }
            trees[idx] = Some(node);
        }
        bound[idx] = (tree_bound_left, tree_bound_right);
        true
    }
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut trees = trees;
        let mut bound = vec![];
        let mut merged = vec![false; trees.len()];
        let mut node_map = std::collections::HashMap::new();
        for (i, tree) in trees.iter().enumerate() {
            if let Some(node) = tree {
                let node = node.borrow();
                node_map.insert(node.val, i);
                let val = node.val;
                let left = node.left.as_ref().map(|n| n.borrow().val).unwrap_or(val);
                let right = node.right.as_ref().map(|n| n.borrow().val).unwrap_or(val);
                bound.push((left, right));
            }
        }
        for i in 0..trees.len() {
            if !Self::merge_tree(i, &node_map, &mut trees, &mut bound, &mut merged, i32::MIN, i32::MAX) {
                return None;
            }
        }
        let mut result = usize::MAX;
        for i in 0..trees.len() {
            if trees[i].is_some() {
                if result == usize::MAX {
                    result = i;
                } else {
                    return None;
                }
            }
        }
        trees[result].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    use itertools::Itertools;
    fn get_tree(v: Vec<Vec<i32>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        v.into_iter().map(|v| TreeNode::from_vec(v)).collect_vec()
    }
    #[test]
    fn test_can_merge() {
        let null = NULL;
        assert_eq!(
            Solution::can_merge(get_tree(vec_vec![[1, null, 3], [2, 1], [3]])),
            TreeNode::from_vec(vec![])
        );
        assert_eq!(
            Solution::can_merge(get_tree(vec_vec![[2, 1], [3, 2, 5], [5, 4]])),
            TreeNode::from_vec(vec![3, 2, 5, 1, null, 4])
        );
        assert_eq!(
            Solution::can_merge(get_tree(vec_vec![[5, 3, 8], [3, 2, 6]])),
            TreeNode::from_vec(vec![])
        );
        assert_eq!(
            Solution::can_merge(get_tree(vec_vec![[5, 4], [3]])),
            TreeNode::from_vec(vec![])
        );
    }
}
