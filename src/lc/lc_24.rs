// https://leetcode.com/problems/swap-nodes-in-pairs/
// 24. Swap Nodes in Pairs
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<Box<ListNode>> = Vec::with_capacity(2);
        let mut new_head = Box::new(ListNode::new(0));
        let mut new_end = &mut new_head;
        loop {
            for _ in 0..2 {
                if let Some(mut node) = head {
                    head = node.as_mut().next.take();
                    vec.push(node);
                } else {
                    break;
                }
            }
            if vec.len() == 2 {
                for node in vec.into_iter().rev() {
                    new_end.next = Some(node);
                    new_end = new_end.next.as_mut()?;
                }
            } else {
                for node in vec {
                    new_end.next = Some(node);
                    new_end = new_end.next.as_mut()?;
                }
                break;
            }
            vec = Vec::new();
        }
        new_head.next
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swap_pairs() {
        assert_eq!(
            Solution::swap_pairs(ListNode::from_vec(vec![1, 2, 3, 4])),
            ListNode::from_vec(vec![2, 1, 4, 3])
        );
        assert_eq!(
            Solution::swap_pairs(ListNode::from_vec(vec![])),
            ListNode::from_vec(vec![])
        );
        assert_eq!(
            Solution::swap_pairs(ListNode::from_vec(vec![1])),
            ListNode::from_vec(vec![1])
        );
    }
}
