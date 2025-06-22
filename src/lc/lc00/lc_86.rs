// https://leetcode.com/problems/partition-list/
// 86. Partition List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut hl = None;
        let mut hr = None;
        while let Some(mut n) = head {
            head = n.next.take();
            if n.val < x {
                n.next = hl;
                hl = Some(n);
            } else {
                n.next = hr;
                hr = Some(n);
            }
        }
        let mut h = None;
        while let Some(mut n) = hr {
            hr = n.next.take();
            n.next = h;
            h = Some(n);
        }
        while let Some(mut n) = hl {
            hl = n.next.take();
            n.next = h;
            h = Some(n);
        }
        h
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition() {
        assert_eq!(
            Solution::partition(ListNode::from_vec(vec![1, 4, 3, 2, 5, 2]), 3),
            ListNode::from_vec(vec![1, 2, 2, 4, 3, 5])
        );
        assert_eq!(
            Solution::partition(ListNode::from_vec(vec![2, 1]), 2),
            ListNode::from_vec(vec![1, 2])
        );
    }
}
