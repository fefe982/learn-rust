// https://leetcode.com/problems/find-duplicate-subtrees/
// 652. Find Duplicate Subtrees

use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    fn traverse_tree(
        root: &Option<Rc<RefCell<TreeNode>>>,
        subtrees: &mut HashMap<Vec<i32>, i32>,
        duptrees: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Vec<i32> {
        let left_tree_mark = -1000;
        let right_tree_mark = -1001;
        let mut vec_tree: Vec<i32> = Vec::new();
        if let Some(r) = root {
            vec_tree.push(r.borrow().val);
            vec_tree.append(&mut Self::traverse_tree(
                &r.borrow().left,
                subtrees,
                duptrees,
            ));
            vec_tree.push(left_tree_mark);
            vec_tree.append(&mut Self::traverse_tree(
                &r.borrow().right,
                subtrees,
                duptrees,
            ));
            vec_tree.push(right_tree_mark);
            if let Some(cnt) = subtrees.get_mut(&vec_tree) {
                if *cnt == 0 {
                    *cnt += 1;
                    duptrees.push(root.clone());
                }
            } else {
                subtrees.insert(vec_tree.clone(), 0);
            }
        }
        vec_tree
    }
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut subtrees: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut duptrees: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        Self::traverse_tree(&root, &mut subtrees, &mut duptrees);
        duptrees
    }
}

#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn find_duplicate_subtrees() {
        assert_eq!(
            Solution::find_duplicate_subtrees(TreeNode::from_vec(vec![
                1, 2, 3, 4, NULL, 2, 4, NULL, NULL, 4
            ])),
            vec![
                TreeNode::from_vec(vec![4]),
                TreeNode::from_vec(vec![2, 4, NULL])
            ]
        );
        assert_eq!(
            Solution::find_duplicate_subtrees(TreeNode::from_vec(vec![2, 1, 1])),
            vec![TreeNode::from_vec(vec![1])]
        );
        assert_eq!(
            Solution::find_duplicate_subtrees(TreeNode::from_vec(vec![2, 2, 2, 3, NULL, 3, NULL])),
            vec![
                TreeNode::from_vec(vec![3]),
                TreeNode::from_vec(vec![2, 3, NULL])
            ]
        );
    }
}
