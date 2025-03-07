// https://leetcode.com/problems/odd-even-linked-list/
// 328. Odd Even Linked List
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut v = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            v.push(Some(node));
        }
        let n = v.len();
        for i in (0..n / 2).rev() {
            let mut node = v[2 * i + 1].take().unwrap();
            node.next = head;
            head = Some(node);
        }
        for i in (0..(n + 1) / 2).rev() {
            let mut node = v[2 * i].take().unwrap();
            node.next = head;
            head = Some(node);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_odd_even_list() {
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![1, 2, 3, 4, 5])),
            ListNode::from_vec(vec![1, 3, 5, 2, 4])
        );
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![2, 1, 3, 5, 6, 4, 7])),
            ListNode::from_vec(vec![2, 3, 6, 7, 1, 5, 4])
        );
    }
}
