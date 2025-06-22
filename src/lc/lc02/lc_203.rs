// https://leetcode.com/problems/remove-linked-list-elements/
// 203. Remove Linked List Elements
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if node.val == val {
                Solution::remove_elements(node.next, val)
            } else {
                node.next = Solution::remove_elements(node.next, val);
                Some(node)
            }
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_elements() {
        assert_eq!(
            Solution::remove_elements(ListNode::from_vec(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            ListNode::from_vec(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::from_vec(vec![]), 1),
            ListNode::from_vec(vec![])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::from_vec(vec![7, 7, 7, 7]), 7),
            ListNode::from_vec(vec![])
        );
    }
}
