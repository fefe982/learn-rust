// https://leetcode.cn/problems/delete-node-in-a-bst/
// 450. Delete Node in a BST
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_left_most_node(root: Rc<RefCell<TreeNode>>) -> (Rc<RefCell<TreeNode>>, Option<Rc<RefCell<TreeNode>>>) {
        let left = root.borrow_mut().left.take();
        if let Some(left) = left {
            let (left, right) = Self::get_left_most_node(left);
            root.borrow_mut().left = right;
            (left, Some(root))
        } else {
            let right = root.borrow_mut().right.take();
            (root, right)
        }
    }
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let val = node.borrow().val;
            match val.cmp(&key) {
                std::cmp::Ordering::Equal => {
                    let mut mnode = node.borrow_mut();
                    if let Some(right) = mnode.right.take() {
                        let left = mnode.left.take();
                        let (root, right) = Self::get_left_most_node(right);
                        root.borrow_mut().right = right;
                        root.borrow_mut().left = left;
                        Some(root)
                    } else {
                        mnode.left.take()
                    }
                }
                std::cmp::Ordering::Less => {
                    let right = node.borrow_mut().right.take();
                    node.borrow_mut().right = Self::delete_node(right, key);
                    Some(node)
                }
                std::cmp::Ordering::Greater => {
                    let left = node.borrow_mut().left.take();
                    node.borrow_mut().left = Self::delete_node(left, key);
                    Some(node)
                }
            }
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    fn check(root: Option<Rc<RefCell<TreeNode>>>, key: i32, res: Option<Rc<RefCell<TreeNode>>>) {
        let ans = Solution::delete_node(root.clone(), key);
        assert_eq!(
            super::super::lc_94::Solution::inorder_traversal(root),
            super::super::lc_94::Solution::inorder_traversal(res.clone())
        );
        assert!(super::super::lc_98::Solution::is_valid_bst(ans));
    }
    #[test]
    fn test_delete_node() {
        let null = NULL;
        check(
            TreeNode::from_vec(vec![5, 3, 6, 2, 4, null, 7]),
            3,
            TreeNode::from_vec(vec![5, 4, 6, 2, null, null, 7]),
        );
        check(
            TreeNode::from_vec(vec![5, 3, 6, 2, 4, null, 7]),
            0,
            TreeNode::from_vec(vec![5, 3, 6, 2, 4, null, 7]),
        );
    }
}
