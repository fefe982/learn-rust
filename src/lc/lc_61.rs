// https://leetcode.com/problems/rotate-list/
// 61. Rotate List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            v.push(Some(node));
        }
        let l = v.len();
        if l == 0 {
            return None;
        }
        let k = (l - k as usize % l) % l;
        head = None;
        for i in (0..l).rev() {
            let mut n = v[(i + k) % l].take().unwrap();
            n.next = head;
            head = Some(n);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_right() {
        assert_eq!(
            Solution::rotate_right(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2),
            ListNode::from_vec(vec![4, 5, 1, 2, 3])
        );
        assert_eq!(
            Solution::rotate_right(ListNode::from_vec(vec![0, 1, 2]), 4),
            ListNode::from_vec(vec![2, 0, 1])
        );
    }
}
