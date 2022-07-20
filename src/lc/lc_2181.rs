// https://leetcode.com/problems/merge-nodes-in-between-zeros/
// 2181. Merge Nodes in Between Zeros

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        let mut head = &head;
        while let Some(node) = head {
            vec.push(node.val);
            head = &node.next;
        }
        vec
    }

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut node = head.as_mut().unwrap();
        for i in vec.iter() {
            node.next = Some(Box::new(ListNode::new(*i)));
            node = node.next.as_mut().unwrap();
        }
        head.unwrap().next
    }

    #[test]
    fn max_score_indices() {
        assert_eq!(
            list_to_vec(Solution::merge_nodes(vec_to_list(vec![
                0, 3, 1, 0, 4, 5, 2, 0
            ]))),
            vec![4, 11]
        );
        assert_eq!(
            list_to_vec(Solution::merge_nodes(vec_to_list(vec![
                0, 1, 0, 3, 0, 2, 2, 0
            ]))),
            vec![1, 3, 4]
        )
    }
}
