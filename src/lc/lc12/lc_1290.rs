// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
// 1290. Convert Binary Number in a Linked List to Integer
pub struct Solution;
use super::super::linked_list::ListNode;
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        fn get_value(head: Option<Box<ListNode>>, v: i32) -> i32 {
            match head {
                Some(node) => get_value(node.next, v << 1 | node.val),
                None => v,
            }
        }
        get_value(head, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_decimal_value() {
        assert_eq!(Solution::get_decimal_value(ListNode::from_vec(vec![1, 0, 1])), 5);
        assert_eq!(Solution::get_decimal_value(ListNode::from_vec(vec![0])), 0);
    }
}
