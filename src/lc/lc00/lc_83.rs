// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// 83. Remove Duplicates from Sorted List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    fn del_dup(head: Option<Box<ListNode>>, last: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if node.val == last {
                return Self::del_dup(node.next, last);
            }
            node.next = Self::del_dup(node.next, node.val);
            Some(node)
        } else {
            head
        }
    }
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::del_dup(head, i32::MIN)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_duplicates() {
        assert_eq!(
            Solution::delete_duplicates(ListNode::from_vec(vec![1, 1, 2])),
            ListNode::from_vec(vec![1, 2])
        );
        assert_eq!(
            Solution::delete_duplicates(ListNode::from_vec(vec![1, 1, 2, 3, 3])),
            ListNode::from_vec(vec![1, 2, 3])
        );
    }
}
