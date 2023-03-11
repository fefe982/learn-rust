// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/description/
// 109. Convert Sorted List to Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use super::linked_list::ListNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = head {
            if let None = node.next {
                Some(Rc::new(RefCell::new(TreeNode::new(node.val))))
            } else {
                let mut sz = 0usize;
                let mut o_node = &head;
                while let Some(ref node) = o_node {
                    o_node = &node.next;
                    sz += 1;
                }
                let mut o_node = &mut head;
                let mut idx: usize = 0;
                while idx + 1 < sz / 2 {
                    o_node = &mut o_node.as_mut().unwrap().next;
                    idx += 1;
                }
                let mut mid = o_node.as_mut().unwrap().next.take();
                let next = mid.as_mut().unwrap().next.take();
                let mut subtree = Some(Rc::new(RefCell::new(TreeNode::new(mid.unwrap().val))));
                subtree.as_mut().unwrap().borrow_mut().left = Self::sorted_list_to_bst(head);
                subtree.as_mut().unwrap().borrow_mut().right = Self::sorted_list_to_bst(next);
                subtree
            }
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorted_list_to_bst() {
        assert_eq!(
            Solution::sorted_list_to_bst(ListNode::from_vec(&vec![1, 9, 10, 15, 19])),
            TreeNode::from_vec(&vec![10, 9, 19, 1, -1, 15])
        );
        assert_eq!(
            Solution::sorted_list_to_bst(ListNode::from_vec(&vec![])),
            TreeNode::from_vec(&vec![])
        );
    }
}
