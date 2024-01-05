// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/
// 2807. Insert Greatest Common Divisors in Linked List
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if a == 0 {
                return b;
            }
            b = b % a;
            if b == 0 {
                return a;
            }
            a = a % b;
        }
    }
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut v = vec![];
        while let Some(node) = head {
            if let Some(lv) = v.last() {
                v.push(Self::gcd(node.val, *lv));
            }
            v.push(node.val);
            head = node.next;
        }
        for v in v.into_iter().rev() {
            head = Some(Box::new(ListNode { val: v, next: head }));
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_greatest_common_divisors() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(ListNode::from_vec(vec![18, 6, 10, 3])),
            ListNode::from_vec(vec![18, 6, 6, 2, 10, 1, 3])
        );
        assert_eq!(
            Solution::insert_greatest_common_divisors(ListNode::from_vec(vec![7])),
            ListNode::from_vec(vec![7])
        );
    }
}
