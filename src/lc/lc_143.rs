// https://leetcode.com/problems/reorder-list/
// 143. Reorder List

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
    fn make_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut node = &mut head;
        for i in vec {
            node.next = Some(Box::new(ListNode::new(i)));
            node = node.next.as_mut().unwrap().as_mut();
        }
        head.next
    }
    #[test]
    fn total_strength() {
        let mut l = make_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, make_list(vec![1, 4, 2, 3]));
        let mut l = make_list(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, make_list(vec![1, 5, 2, 4, 3]));
    }
}
