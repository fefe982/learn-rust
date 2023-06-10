// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// 1171. Remove Zero Sum Consecutive Nodes from Linked List
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = Vec::new();
        let mut sum = std::collections::HashMap::<i32, usize>::new();
        sum.insert(0, 0);
        let mut sum_list = Vec::new();
        let mut node = head;
        let mut s = 0;
        while let Some(mut n) = node {
            s += n.val;
            node = n.as_mut().next.take();
            if let Some(&idx) = sum.get(&s) {
                for i in idx..sum_list.len() {
                    sum.remove(&sum_list[i]);
                }
                list.resize_with(idx, || panic!(""));
                sum_list.resize(idx, 0);
            } else {
                list.push(n);
                sum_list.push(s);
                sum.insert(s, list.len());
            }
        }
        while list.len() > 1 {
            let tail = list.pop();
            list.last_mut().unwrap().as_mut().next = tail;
        }
        list.pop()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_zero_sum_sublists() {
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(vec![1, 2, -3, 3, 1])),
            ListNode::from_vec(vec![3, 1])
        );
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(vec![1, 2, 3, -3, 4])),
            ListNode::from_vec(vec![1, 2, 4])
        );
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(vec![1, 2, 3, -3, -2])),
            ListNode::from_vec(vec![1])
        );
    }
}
