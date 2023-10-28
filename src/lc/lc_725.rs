// https://leetcode.com/problems/split-linked-list-in-parts/
// 725. Split Linked List in Parts
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut v = vec![];
        let mut onode = head;
        while let Some(mut node) = onode {
            onode = node.next.take();
            v.push(Some(node));
        }
        let mut res = vec![];
        let len = v.len() / k as usize;
        let rest = v.len() % k as usize;
        let mut idx = 0;
        for i in 0..k as usize {
            let start = idx;
            let mut end = start + len;
            if i < rest {
                end += 1;
            }
            let mut h = None;
            for j in (start..end).rev() {
                let mut nh = v[j].take();
                nh.as_mut().unwrap().next = h.take();
                h = nh;
            }
            res.push(h);
            idx = end;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_list_to_parts() {
        assert_eq!(
            Solution::split_list_to_parts(ListNode::from_vec(vec![1, 2, 3]), 5),
            vec![
                ListNode::from_vec(vec![1]),
                ListNode::from_vec(vec![2]),
                ListNode::from_vec(vec![3]),
                ListNode::from_vec(vec![]),
                ListNode::from_vec(vec![])
            ]
        );
        assert_eq!(
            Solution::split_list_to_parts(
                ListNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
                3
            ),
            vec![
                ListNode::from_vec(vec![1, 2, 3, 4]),
                ListNode::from_vec(vec![5, 6, 7]),
                ListNode::from_vec(vec![8, 9, 10])
            ]
        );
    }
}
