// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/
// 3217. Delete Nodes From Linked List in an Array
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut head = head;
        let set: std::collections::HashSet<i32> = std::collections::HashSet::from_iter(nums);
        while let Some(mut node) = head {
            head = node.next.take();
            if !set.contains(&node.val) {
                v.push(node);
            }
        }
        head = None;
        for mut node in v.into_iter().rev() {
            node.next = head;
            head = Some(node);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modified_list() {
        assert_eq!(
            Solution::modified_list(vec![1, 2, 3], ListNode::from_vec(vec![1, 2, 3, 4, 5])),
            ListNode::from_vec(vec![4, 5])
        );
        assert_eq!(
            Solution::modified_list(vec![1], ListNode::from_vec(vec![1, 2, 1, 2, 1, 2])),
            ListNode::from_vec(vec![2, 2, 2])
        );
        assert_eq!(
            Solution::modified_list(vec![5], ListNode::from_vec(vec![1, 2, 3, 4])),
            ListNode::from_vec(vec![1, 2, 3, 4])
        );
    }
}
