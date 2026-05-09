// https://leetcode.com/problems/reverse-nodes-in-even-length-groups/
// 2074. Reverse Nodes in Even Length Groups
use super::super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = Vec::new();
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            vals.push(node.val);
            cur = node.next.as_ref();
        }

        let n = vals.len();
        let mut start = 0usize;
        let mut group_len = 1usize;

        while start < n {
            let end = (start + group_len).min(n);
            let len = end - start;
            if len % 2 == 0 {
                vals[start..end].reverse();
            }
            start = end;
            group_len += 1;
        }

        let mut ans: Option<Box<ListNode>> = None;
        for &v in vals.iter().rev() {
            ans = Some(Box::new(ListNode { val: v, next: ans }));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_even_length_groups() {
        assert_eq!(
            Solution::reverse_even_length_groups(ListNode::from_vec(vec![5, 2, 6, 3, 9, 1, 7, 3, 8, 4])),
            ListNode::from_vec(vec![5, 6, 2, 3, 9, 1, 4, 8, 3, 7])
        );
        assert_eq!(
            Solution::reverse_even_length_groups(ListNode::from_vec(vec![1, 1, 0, 6])),
            ListNode::from_vec(vec![1, 0, 1, 6])
        );
        assert_eq!(
            Solution::reverse_even_length_groups(ListNode::from_vec(vec![1])),
            ListNode::from_vec(vec![1])
        );
    }
}
