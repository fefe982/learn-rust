// https://leetcode.com/problems/merge-in-between-linked-lists/
// 1669. Merge In Between Linked Lists
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut idx = 0;
        let mut list1 = list1;
        let mut list2 = list2;
        while idx < a {
            let mut node = list1;
            list1 = node.as_mut().unwrap().next.take();
            v.push(node);
            idx += 1;
        }
        while list2.is_some() {
            let mut node = list2;
            list2 = node.as_mut().unwrap().next.take();
            v.push(node);
        }
        while idx <= b {
            let mut node = list1;
            list1 = node.as_mut().unwrap().next.take();
            idx += 1;
        }
        while list1.is_some() {
            let mut node = list1;
            list1 = node.as_mut().unwrap().next.take();
            v.push(node);
        }
        let mut head = None;
        while let Some(mut node) = v.pop() {
            node.as_mut().unwrap().next = head;
            head = node;
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_in_between() {
        assert_eq!(
            Solution::merge_in_between(
                ListNode::from_vec(vec![10, 1, 13, 6, 9, 5]),
                3,
                4,
                ListNode::from_vec(vec![1000000, 1000001, 1000002])
            ),
            ListNode::from_vec(vec![10, 1, 13, 1000000, 1000001, 1000002, 5])
        );
        assert_eq!(
            Solution::merge_in_between(
                ListNode::from_vec(vec![0, 1, 2, 3, 4, 5, 6]),
                2,
                5,
                ListNode::from_vec(vec![1000000, 1000001, 1000002, 1000003, 1000004])
            ),
            ListNode::from_vec(vec![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6])
        );
    }
}
