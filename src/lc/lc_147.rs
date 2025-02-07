// https://leetcode.com/problems/insertion-sort-list/
// 147. Insertion Sort List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    fn insert(head: Option<Box<ListNode>>, mut node: Box<ListNode>) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            if head.val > node.val {
                node.next = Some(head);
                return Some(node);
            }
            head.next = Self::insert(head.next, node);
            return Some(head);
        } else {
            return Some(node);
        }
    }
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = head;
        let mut head = None;
        while let Some(mut node) = tail {
            tail = node.next.take();
            head = Self::insert(head, node);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_list() {
        assert_eq!(
            Solution::insertion_sort_list(ListNode::from_vec(vec![4, 2, 1, 3])),
            ListNode::from_vec(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::insertion_sort_list(ListNode::from_vec(vec![-1, 5, 3, 4, 0])),
            ListNode::from_vec(vec![-1, 0, 3, 4, 5])
        );
    }
}
