// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/
// 2816. Double a Number Represented as a Linked List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = vec![];
        let mut head = head;
        while let Some(node) = head {
            n.push(node.val);
            head = node.next;
        }
        let mut carry = 0;
        for i in (0..n.len()).rev() {
            let mul = n[i] * 2 + carry;
            n[i] = mul % 10;
            carry = mul / 10;
        }
        head = None;
        for i in n.into_iter().rev() {
            head = Some(Box::new(ListNode { val: i, next: head }));
        }
        if carry > 0 {
            Some(Box::new(ListNode { val: carry, next: head }))
        } else {
            head
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double_it() {
        assert_eq!(
            Solution::double_it(ListNode::from_vec(vec![1, 8, 9])),
            ListNode::from_vec(vec![3, 7, 8])
        );
        assert_eq!(
            Solution::double_it(ListNode::from_vec(vec![9, 9, 9])),
            ListNode::from_vec(vec![1, 9, 9, 8])
        );
    }
}
