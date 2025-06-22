// https://leetcode.com/problems/reverse-linked-list/
// 206. Reverse Linked List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rev = None;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = rev;
            rev = Some(node);
        }
        rev
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_list() {
        assert_eq!(
            Solution::reverse_list(ListNode::from_vec(vec![])),
            ListNode::from_vec(vec![])
        );
        assert_eq!(
            Solution::reverse_list(ListNode::from_vec(vec![1, 2, 3, 4, 5])),
            ListNode::from_vec(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_list(ListNode::from_vec(vec![1, 2])),
            ListNode::from_vec(vec![2, 1])
        );
    }
}
