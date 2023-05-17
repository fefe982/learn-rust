// https://leetcode.com/problems/next-greater-node-in-linked-list/
// 1019. Next Greater Node In Linked List
pub struct Solution;
use super::linked_list::*;
impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut node = head;
        while let Some(n) = node {
            v.push(n.val);
            node = n.next;
        }
        let mut vmax = vec![];
        for i in (0..v.len()).rev() {
            let p = vmax.partition_point(|&x| x > v[i]);
            let m = if p == 0 { 0 } else { vmax[p - 1] };
            if p == vmax.len() {
                vmax.push(v[i])
            } else {
                vmax[p] = v[i];
                vmax.resize(p + 1, 0);
            }
            v[i] = m;
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_larger_nodes() {
        assert_eq!(
            Solution::next_larger_nodes(ListNode::from_vec(vec![2, 1, 5])),
            vec![5, 5, 0]
        );
        assert_eq!(
            Solution::next_larger_nodes(ListNode::from_vec(vec![2, 7, 4, 3, 5])),
            vec![7, 0, 5, 5, 0]
        );
    }
}
