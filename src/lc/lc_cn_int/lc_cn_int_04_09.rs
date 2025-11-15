// https://leetcode.cn/problems/bst-sequences-lcci/
// 面试题 04.09. BST Sequences LCCI
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn seq(nodes: Vec<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>, prefix: Vec<i32>) {
        if nodes.is_empty() {
            res.push(prefix);
            return;
        }
        for i in 0..nodes.len() {
            let mut prefix = prefix.clone();
            let node = &nodes[i];
            prefix.push(node.borrow().val);
            let mut nodes = nodes.clone();
            nodes.remove(i);
            if node.borrow().left.is_some() {
                nodes.push(node.borrow().left.clone().unwrap());
            }
            if node.borrow().right.is_some() {
                nodes.push(node.borrow().right.clone().unwrap());
            }
            Self::seq(nodes, res, prefix);
        }
    }
    pub fn bst_sequences(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if let Some(node) = root {
            Self::seq(vec![node], &mut res, vec![]);
        } else {
            res.push(vec![]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn bst_sequences() {
        assert_eq!(
            Solution::bst_sequences(TreeNode::from_vec(vec![2, 1, 3])),
            vec_vec![[2, 1, 3], [2, 3, 1]]
        );
    }
}
