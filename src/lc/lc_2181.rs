// https://leetcode.com/problems/merge-nodes-in-between-zeros/
// 2181. Merge Nodes in Between Zeros
use super::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode::new(0)));
        let mut new_node = new_head.as_mut().unwrap();
        let mut head = &head;
        let mut sum = 0;
        while let Some(node) = head {
            if node.val == 0 {
                if sum != 0 {
                    new_node.next = Some(Box::new(ListNode::new(sum)));
                    sum = 0;
                    new_node = new_node.next.as_mut().unwrap()
                }
            } else {
                sum += node.val
            }
            head = &node.next
        }
        new_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score_indices() {
        assert_eq!(
            Solution::merge_nodes(ListNode::from_vec(&vec![0, 3, 1, 0, 4, 5, 2, 0])),
            ListNode::from_vec(&vec![4, 11])
        );
        assert_eq!(
            Solution::merge_nodes(ListNode::from_vec(&vec![0, 1, 0, 3, 0, 2, 2, 0])),
            ListNode::from_vec(&vec![1, 3, 4])
        )
    }
}
