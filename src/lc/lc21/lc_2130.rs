// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/
// 2130. Maximum Twin Sum of a Linked List
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut v = Vec::new();
        while let Some(n) = head {
            v.push(n.val);
            head = n.next;
        }
        let mut m = 0;
        let l = v.len();
        for i in 0..(l / 2) {
            m = std::cmp::max(m, v[i] + v[l - i - 1]);
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pair_sum() {
        assert_eq!(Solution::pair_sum(ListNode::from_vec(vec![5, 4, 2, 1])), 6);
        assert_eq!(Solution::pair_sum(ListNode::from_vec(vec![4, 2, 2, 3])), 7);
        assert_eq!(
            Solution::pair_sum(ListNode::from_vec(vec![1, 100000])),
            100001
        );
    }
}
