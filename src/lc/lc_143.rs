// https://leetcode.com/problems/reorder-list/
// 143. Reorder List
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut nodes = vec![head.take()];
        loop {
            let next = nodes.last_mut().unwrap().as_mut().unwrap().next.take();
            if let None = next {
                break;
            }
            nodes.push(next);
        }
        let len = nodes.len();
        let mut node = head;
        for l in 0..len / 2 {
            node.replace(nodes[l].take().unwrap());
            node = &mut node.as_mut().unwrap().next;
            node.replace(nodes[len - l - 1].take().unwrap());
            node = &mut node.as_mut().unwrap().next;
        }
        if len % 2 == 1 {
            node.replace(nodes[len / 2].take().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_strength() {
        let mut l = ListNode::from_vec(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, ListNode::from_vec(vec![1, 4, 2, 3]));
        let mut l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, ListNode::from_vec(vec![1, 5, 2, 4, 3]));
    }
}
