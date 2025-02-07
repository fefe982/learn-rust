// https://leetcode.com/problems/sort-list/
// 148. Sort List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut v = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            v.push(node);
        }
        v.sort_by(|a, b| a.val.cmp(&b.val));
        let mut head = None;
        for mut node in v.into_iter().rev() {
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
    fn sort_list() {
        assert_eq!(
            Solution::sort_list(ListNode::from_vec(vec![4, 2, 1, 3])),
            ListNode::from_vec(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::sort_list(ListNode::from_vec(vec![-1, 5, 3, 4, 0])),
            ListNode::from_vec(vec![-1, 0, 3, 4, 5])
        );
    }
}
