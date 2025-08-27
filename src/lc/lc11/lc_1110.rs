// https://leetcode.com/problems/delete-nodes-and-return-forest/
// 1110. Delete Nodes And Return Forest
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    fn del_node_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &Vec<i32>,
        vec: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        is_root: bool,
    ) -> bool {
        let mut delete_this = true;
        if let Some(node) = root {
            delete_this = to_delete.contains(&node.borrow().val);
            if is_root && !delete_this {
                vec.push(Some(node.clone()))
            }
            if Self::del_node_helper(node.borrow().left.clone(), to_delete, vec, delete_this) {
                node.borrow_mut().left = None;
            }
            if Self::del_node_helper(node.borrow().right.clone(), to_delete, vec, delete_this) {
                node.borrow_mut().right = None;
            }
        }
        delete_this
    }
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut vec = Vec::new();
        Self::del_node_helper(root, &to_delete, &mut vec, true);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn del_nodes() {
        assert_eq!(
            Solution::del_nodes(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7]), vec![3, 5]),
            vec![
                TreeNode::from_vec(vec![1, 2, NULL, 4]),
                TreeNode::from_vec(vec![6]),
                TreeNode::from_vec(vec![7])
            ]
        );
        assert_eq!(
            Solution::del_nodes(TreeNode::from_vec(vec![1, 2, 4, NULL, 3]), vec![3]),
            vec![TreeNode::from_vec(vec![1, 2, 4])]
        );
    }
}
