// https://leetcode.com/problems/middle-of-the-linked-list/
// 876. Middle of the Linked List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_middle_node() {
        assert_eq!(
            Solution::middle_node(ListNode::from_vec(vec![1, 2, 3, 4, 5])),
            ListNode::from_vec(vec![3, 4, 5])
        );
        assert_eq!(
            Solution::middle_node(ListNode::from_vec(vec![1, 2, 3, 4, 5, 6])),
            ListNode::from_vec(vec![4, 5, 6])
        );
    }
}
