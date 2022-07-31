// https://leetcode.com/problems/linked-list-in-binary-tree/
// 1367. Linked List in Binary Tree
use super::binary_tree::TreeNode;
use super::linked_list::ListNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    fn match_sub_path(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match head {
            None => true,
            Some(list_node) => match root {
                None => false,
                Some(tree_node) => {
                    list_node.val == tree_node.borrow().val
                        && (Self::match_sub_path(&list_node.next, &tree_node.borrow().left)
                            || Self::match_sub_path(&list_node.next, &tree_node.borrow().right))
                }
            },
        }
    }
    fn traverse_tree(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => false,
            Some(node) => {
                Self::match_sub_path(head, root)
                    || Self::traverse_tree(head, &node.borrow().left)
                    || Self::traverse_tree(head, &node.borrow().right)
            }
        }
    }
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::traverse_tree(&head, &root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_sub_path() {
        assert_eq!(
            Solution::is_sub_path(
                ListNode::from_vec(&vec![4, 2, 8]),
                TreeNode::from_vec(&vec![
                    1, 4, 4, -1, 2, 2, -1, 1, -1, 6, 8, -1, -1, -1, -1, 1, 3
                ])
            ),
            true
        );
        assert_eq!(
            Solution::is_sub_path(
                ListNode::from_vec(&vec![1, 4, 2, 6]),
                TreeNode::from_vec(&vec![
                    1, 4, 4, -1, 2, 2, -1, 1, -1, 6, 8, -1, -1, -1, -1, 1, 3
                ])
            ),
            true
        );
        assert_eq!(
            Solution::is_sub_path(
                ListNode::from_vec(&vec![1, 4, 2, 6, 8]),
                TreeNode::from_vec(&vec![
                    1, 4, 4, -1, 2, 2, -1, 1, -1, 6, 8, -1, -1, -1, -1, 1, 3
                ])
            ),
            false
        );
    }
}
