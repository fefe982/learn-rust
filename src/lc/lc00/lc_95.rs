// https://leetcode.com/problems/unique-binary-search-trees-ii/
// 95. Unique Binary Search Trees II
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn gen_tree(from: i32, to: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut res = vec![];
        for i in from..=to {
            let left = if i > from {
                Self::gen_tree(from, i - 1)
            } else {
                vec![None]
            };
            let right = if to > i {
                Self::gen_tree(i + 1, to)
            } else {
                vec![None]
            };
            for l in left {
                for r in &right {
                    let mut t = TreeNode::new(i);
                    t.left = l.clone();
                    t.right = r.clone();
                    res.push(Some(Rc::new(RefCell::new(t))));
                }
            }
        }
        res
    }
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::gen_tree(1, n)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn generate_trees() {
        let null = NULL;
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                TreeNode::from_vec(vec![1, null, 2, null, 3]),
                TreeNode::from_vec(vec![1, null, 3, 2]),
                TreeNode::from_vec(vec![2, 1, 3]),
                TreeNode::from_vec(vec![3, 1, null, null, 2]),
                TreeNode::from_vec(vec![3, 2, null, 1])
            ]
        );
        assert_eq!(
            Solution::generate_trees(1),
            vec![TreeNode::from_vec(vec![1])]
        );
    }
}
