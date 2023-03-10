// https://leetcode.com/problems/merge-k-sorted-lists/
// 23. Merge k Sorted Lists

pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    fn merge_list(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
        merged_head: &mut ListNode,
    ) {
        let mut cur = merged_head;
        loop {
            if let Some(ref head1) = list1 {
                if let Some(ref head2) = list2 {
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
            } else {
                break;
            }
        }
        if let None = list1 {
            cur.next = list2;
        } else {
            cur.next = list1;
        }
    }
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut merged: Option<Box<ListNode>> = None;
        let mut merged_head = ListNode::new(0);
        for v in lists {
            Self::merge_list(merged, v, &mut merged_head);
            merged = merged_head.next.take();
        }
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_k_lists() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                ListNode::from_vec(&vec![1, 4, 5]),
                ListNode::from_vec(&vec![1, 3, 4]),
                ListNode::from_vec(&vec![2, 6])
            ]),
            ListNode::from_vec(&vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), ListNode::from_vec(&vec![]));
        assert_eq!(
            Solution::merge_k_lists(vec![ListNode::from_vec(&vec![])]),
            ListNode::from_vec(&vec![])
        );
    }
}
