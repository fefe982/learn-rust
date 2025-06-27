// https://leetcode.com/problems/all-possible-full-binary-trees/
// 894. All Possible Full Binary Trees
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        let tot = n as usize / 2 + 1;
        let mut res = vec![];
        res.push(vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]);
        for i in 1..tot {
            let mut treei = vec![];
            for j in 0..i {
                for m in 0..res[j].len() {
                    for n in 0..res[i - j - 1].len() {
                        let mut root = TreeNode::new(0);
                        root.left = res[j][m].clone();
                        root.right = res[i - j - 1][n].clone();
                        treei.push(Some(Rc::new(RefCell::new(root))));
                    }
                }
            }
            res.push(treei);
        }
        res[tot - 1].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn all_possible_fbt() {
        let null = NULL;
        assert_eq!(
            Solution::all_possible_fbt(7),
            vec![
                TreeNode::from_vec(vec![0, 0, 0, null, null, 0, 0, null, null, 0, 0]),
                TreeNode::from_vec(vec![0, 0, 0, null, null, 0, 0, 0, 0]),
                TreeNode::from_vec(vec![0, 0, 0, 0, 0, 0, 0]),
                TreeNode::from_vec(vec![0, 0, 0, 0, 0, null, null, null, null, 0, 0]),
                TreeNode::from_vec(vec![0, 0, 0, 0, 0, null, null, 0, 0])
            ]
        );
        assert_eq!(
            Solution::all_possible_fbt(3),
            vec![TreeNode::from_vec(vec![0, 0, 0])]
        );
    }
}
