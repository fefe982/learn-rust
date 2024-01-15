// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
// 82. Remove Duplicates from Sorted List II
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    fn del_dup(head: Option<Box<ListNode>>, last_node: Option<Box<ListNode>>, last_val: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if node.val == last_val {
                return Self::del_dup(node.next, None, last_val);
            }
            let v = node.val;
            let n = Self::del_dup(node.next.take(), Some(node), v);
            if let Some(mut ln) = last_node {
                ln.next = n;
                Some(ln)
            } else {
                n
            }
        } else {
            last_node
        }
    }
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::del_dup(head, None, i32::MIN)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_duplicates() {
        assert_eq!(
            Solution::delete_duplicates(ListNode::from_vec(vec![1, 2, 3, 3, 4, 4, 5])),
            ListNode::from_vec(vec![1, 2, 5])
        );
        assert_eq!(
            Solution::delete_duplicates(ListNode::from_vec(vec![1, 1, 1, 2, 3])),
            ListNode::from_vec(vec![2, 3])
        );
    }
}
