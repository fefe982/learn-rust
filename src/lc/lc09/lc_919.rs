// https://leetcode.com/problems/complete-binary-tree-inserter/
// 919. Complete Binary Tree Inserter
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    queue: VecDeque<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut lvl = 1;
        let mut q0 = vec![];
        let mut q1 = vec![];
        fn dfs(
            root: &Rc<RefCell<TreeNode>>,
            level: i32,
            lvl_max: &mut i32,
            q0: &mut Vec<Rc<RefCell<TreeNode>>>,
            q1: &mut Vec<Rc<RefCell<TreeNode>>>,
        ) {
            *lvl_max = level.max(*lvl_max);
            if root.borrow().left.is_none() {
                if level == *lvl_max {
                    q0.push(root.clone());
                } else {
                    q1.push(root.clone());
                }
            } else {
                dfs(&root.borrow().left.as_ref().unwrap(), level + 1, lvl_max, q0, q1);
                if root.borrow().right.is_none() {
                    if level == *lvl_max {
                        q0.push(root.clone());
                    } else {
                        q1.push(root.clone());
                    }
                } else {
                    dfs(&root.borrow().right.as_ref().unwrap(), level + 1, lvl_max, q0, q1);
                }
            }
        }
        dfs(root.as_ref().unwrap(), 1, &mut lvl, &mut q0, &mut q1);
        Self {
            root: root,
            queue: q1.into_iter().chain(q0.into_iter()).collect(),
        }
    }

    pub fn insert(&mut self, val: i32) -> i32 {
        let node = self.queue.pop_front().unwrap();
        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
        let val = node.borrow().val;
        if node.borrow().left.is_none() {
            node.borrow_mut().left = Some(new_node.clone());
            self.queue.push_front(node);
        } else {
            node.borrow_mut().right = Some(new_node.clone());
        }
        self.queue.push_back(new_node);
        val
    }

    pub fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_919() {
        let mut obj = CBTInserter::new(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6]));
        assert_eq!(obj.insert(7), 3);
        assert_eq!(obj.insert(8), 4);
        assert_eq!(obj.get_root(), TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    }
}
