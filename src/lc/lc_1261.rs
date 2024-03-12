// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
// 1261. Find Elements in a Contaminated Binary Tree
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct FindElements {
    tree: std::collections::HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn update_tree(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, set: &mut std::collections::HashSet<i32>) {
        if let Some(node) = root {
            set.insert(val);
            let node = node.borrow();
            Self::update_tree(&node.left, val * 2 + 1, set);
            Self::update_tree(&node.right, val * 2 + 2, set);
        }
    }
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut set = std::collections::HashSet::new();
        Self::update_tree(&root, 0, &mut set);
        Self { tree: set }
    }

    pub fn find(&self, target: i32) -> bool {
        self.tree.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    #[test]
    fn test_find_elements() {
        let null = NULL;
        for (init, find, expect) in [
            (vec![-1, null, -1], vec_vec![[1], [2]], vec![false, true]),
            (
                vec![-1, -1, -1, -1, -1],
                vec_vec![[1], [3], [5]],
                vec![true, true, false],
            ),
            (
                vec![-1, null, -1, -1, null, -1],
                vec_vec![[2], [3], [4], [5]],
                vec![true, false, false, true],
            ),
        ] {
            let obj = FindElements::new(TreeNode::from_vec(init));
            for (target, expect) in find.into_iter().zip(expect.into_iter()) {
                assert_eq!(obj.find(target[0]), expect);
            }
        }
    }
}
