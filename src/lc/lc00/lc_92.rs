// https://leetcode.com/problems/reverse-linked-list-ii/
// 92. Reverse Linked List II
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        let mut nhead = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut nhead;
        let mut tail = head;
        for _ in 1..left {
            let ntail = tail.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = tail.take();
            cur = &mut cur.as_mut().unwrap().next;
            tail = ntail;
        }
        let mut mid = None;
        for _ in left..=right {
            let ntail = tail.as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = mid.take();
            mid = tail;
            tail = ntail;
        }
        cur.as_mut().unwrap().next = mid;
        while !cur.as_mut().unwrap().next.is_none() {
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur.as_mut().unwrap().next = tail;
        nhead.unwrap().next
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2, 4),
            ListNode::from_vec(vec![1, 4, 3, 2, 5])
        );
        assert_eq!(
            Solution::reverse_between(ListNode::from_vec(vec![5]), 1, 1),
            ListNode::from_vec(vec![5])
        );
    }
}
