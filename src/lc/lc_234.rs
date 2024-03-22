// https://leetcode.com/problems/palindrome-linked-list/
// 234. Palindrome Linked List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut v = vec![];
        let mut p = head;
        while let Some(node) = p {
            v.push(node.val);
            p = node.next;
        }
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(ListNode::from_vec(vec![1, 2, 2, 1])), true);
        assert_eq!(Solution::is_palindrome(ListNode::from_vec(vec![1, 2])), false);
    }
}
