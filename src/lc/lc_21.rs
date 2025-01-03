// https://leetcode.com/problems/merge-two-sorted-lists/
// 21. Merge Two Sorted Lists
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut head = ListNode::new(0);
        let mut cur = &mut head;
        loop {
            if let (Some(head1), Some(head2)) = (&list1, &list2) {
                if head1.val < head2.val {
                    let mut head1 = list1;
                    list1 = head1.as_mut().unwrap().next.take();
                    cur.next = head1;
                    cur = cur.next.as_mut().unwrap();
                } else {
                    let mut head2 = list2;
                    list2 = head2.as_mut().unwrap().next.take();
                    cur.next = head2;
                    cur = cur.next.as_mut().unwrap();
                }
            } else {
                break;
            }
        }
        if let None = list1 {
            cur.next = list2;
        } else {
            cur.next = list1;
        }
        head.next
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_two_lists() {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(vec![1, 2, 4]), ListNode::from_vec(vec![1, 3, 4])),
            ListNode::from_vec(vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![])),
            ListNode::from_vec(vec![])
        );
    }
}
