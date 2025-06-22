// https://leetcode.com/problems/reverse-nodes-in-k-group/
// 25. Reverse Nodes in k-Group
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut vec: Vec<Box<ListNode>> = Vec::with_capacity(k as usize);
        let mut new_head = Box::new(ListNode::new(0));
        let mut new_end = &mut new_head;
        loop {
            for _ in 0..k {
                if let Some(mut node) = head {
                    head = node.as_mut().next.take();
                    vec.push(node);
                } else {
                    break;
                }
            }
            if vec.len() == k as usize {
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
    fn reverse_k_group() {
        assert_eq!(
            Solution::reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2),
            ListNode::from_vec(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 3),
            ListNode::from_vec(vec![3, 2, 1, 4, 5])
        );
    }
}
