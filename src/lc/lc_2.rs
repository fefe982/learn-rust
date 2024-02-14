// https://leetcode.com/problems/add-two-numbers/
// 2. Add Two Numbers
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    fn next(l: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        if let Some(mut n) = l {
            (n.val, n.next.take())
        } else {
            (0, None)
        }
    }
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut r = vec![];
        let mut c = 0;
        loop {
            let (v1, nl1) = Self::next(l1);
            let (v2, nl2) = Self::next(l2);
            l1 = nl1;
            l2 = nl2;
            let d = v1 + v2 + c;
            r.push(d % 10);
            c = d / 10;
            if l1 == None && l2 == None && c == 0 {
                break;
            }
        }
        let mut head = None;
        for v in r.into_iter().rev() {
            let mut n = Box::new(ListNode::new(v));
            n.next = head;
            head = Some(n);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_vec(vec![2, 4, 3]), ListNode::from_vec(vec![5, 6, 4])),
            ListNode::from_vec(vec![7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0])),
            ListNode::from_vec(vec![0])
        );
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_vec(vec![9, 9, 9, 9])
            ),
            ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
