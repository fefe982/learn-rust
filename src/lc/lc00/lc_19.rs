// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// 19. Remove Nth Node From End of List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut cur = head;
        while let Some(node) = cur {
            v.push(node.val);
            cur = node.next;
        }
        v.remove(v.len() - n as usize);
        let mut head = None;
        for n in v.into_iter().rev() {
            let mut node = ListNode::new(n);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2),
            ListNode::from_vec(vec![1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1]), 1),
            ListNode::from_vec(vec![])
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1, 2]), 1),
            ListNode::from_vec(vec![1])
        );
    }
}
