// https://leetcode.com/problems/remove-nodes-from-linked-list/
// 2487. Remove Nodes From Linked List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut nodes = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            nodes.push(node);
        }
        let mut fnodes = std::collections::VecDeque::<Box<ListNode>>::new();
        while let Some(node) = nodes.pop() {
            if let Some(prev) = fnodes.back() {
                if prev.val <= node.val {
                    fnodes.push_back(node);
                }
            } else {
                fnodes.push_back(node);
            }
        }
        head = fnodes.pop_front();
        while let Some(mut node) = fnodes.pop_front() {
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
    fn test_remove_nodes() {
        assert_eq!(
            Solution::remove_nodes(ListNode::from_vec(vec![5, 2, 13, 3, 8])),
            ListNode::from_vec(vec![13, 8])
        );
        assert_eq!(
            Solution::remove_nodes(ListNode::from_vec(vec![1, 1, 1, 1])),
            ListNode::from_vec(vec![1, 1, 1, 1])
        );
    }
}
