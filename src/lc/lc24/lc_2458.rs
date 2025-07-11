// https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/
// 2458. Height of Binary Tree After Subtree Removal Queries
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        mem: &mut Vec<(i32, i32)>,
        val: &mut Vec<(usize, i32)>,
    ) -> i32 {
        if let Some(r) = root {
            let v = r.borrow().val as usize;
            let l = Self::walk(&r.borrow().left, depth + 1, mem, val);
            let r = Self::walk(&r.borrow().right, depth + 1, mem, val);
            let d = l.max(r) + 1;
            if mem.len() <= depth {
                mem.resize(depth + 1, (0, 0));
            }
            let (d0, d1) = mem[depth];
            if d > d0 {
                mem[depth] = (d, d0);
            } else if d > d1 {
                mem[depth] = (d0, d);
            }
            if val.len() <= v {
                val.resize(v + 1, (0, 0));
            }
            val[v as usize] = (depth, d);
            d
        } else {
            0
        }
    }
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut mem = vec![];
        let mut val = vec![];
        Self::walk(&root, 0, &mut mem, &mut val);
        queries
            .into_iter()
            .map(|q| {
                let (d, h) = val[q as usize];
                let (h0, h1) = mem[d];
                d as i32 + if h == h0 { h1 } else { h0 } - 1
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_tree_queries() {
        let null = NULL;
        assert_eq!(
            Solution::tree_queries(
                TreeNode::from_vec(vec![1, 3, 4, 2, null, 6, 5, null, null, null, null, null, 7]),
                vec![4]
            ),
            [2]
        );
        assert_eq!(
            Solution::tree_queries(TreeNode::from_vec(vec![5, 8, 9, 2, 1, 3, 7, 4, 6]), vec![3, 2, 4, 8]),
            [3, 2, 3, 2]
        );
    }
}
