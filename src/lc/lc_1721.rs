// https://leetcode.com/problems/swapping-nodes-in-a-linked-list/
// 1721. Swapping Nodes in a Linked List
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        while let Some(n) = head {
            v.push(n.as_ref().val);
            head = n.next;
        }
        let l = v.len();
        let v1 = v[k as usize - 1];
        v[k as usize - 1] = v[l - k as usize];
        v[l - k as usize] = v1;
        let mut head = ListNode::new(0);
        let mut node = &mut head;
        for i in v {
            node.next = Some(Box::new(ListNode::new(i)));
            node = node.next.as_mut().unwrap().as_mut();
        }
        head.next
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swap_nodes() {
        assert_eq!(
            Solution::swap_nodes(ListNode::from_vec(&vec![1, 2, 3, 4, 5]), 2),
            ListNode::from_vec(&vec![1, 4, 3, 2, 5])
        );
        assert_eq!(
            Solution::swap_nodes(ListNode::from_vec(&vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]), 5),
            ListNode::from_vec(&vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5])
        );
    }
}
