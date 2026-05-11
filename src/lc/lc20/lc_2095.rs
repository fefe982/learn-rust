// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/
// 2095. Delete the Middle Node of a Linked List
use super::super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = Vec::new();
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            vals.push(node.val);
            cur = node.next.as_ref();
        }
        if vals.is_empty() {
            return None;
        }
        vals.remove(vals.len() / 2);
        let mut ans: Option<Box<ListNode>> = None;
        for &v in vals.iter().rev() {
            ans = Some(Box::new(ListNode { val: v, next: ans }));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_delete_middle() {
        assert_eq!(
            Solution::delete_middle(linked_list![1, 3, 4, 7, 1, 2, 6]),
            linked_list![1, 3, 4, 1, 2, 6]
        );
        assert_eq!(Solution::delete_middle(linked_list![1, 2, 3, 4]), linked_list![1, 2, 4]);
        assert_eq!(Solution::delete_middle(linked_list![2, 1]), linked_list![2]);
        assert_eq!(Solution::delete_middle(linked_list![1]), None);
    }
}
