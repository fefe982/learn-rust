// https://leetcode.com/problems/linked-list-components/
// 817. Linked List Components
pub struct Solution;
use super::super::linked_list::ListNode;
impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut v = vec![0; 10001];
        for n in nums {
            v[n as usize] = 1;
        }
        let mut c = false;
        let mut cnt = 0;
        let mut head = head;
        while let Some(node) = head {
            if v[node.val as usize] == 1 {
                if !c {
                    cnt += 1;
                    c = true;
                }
            } else {
                c = false;
            }
            head = node.next;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_components() {
        assert_eq!(
            Solution::num_components(ListNode::from_vec(vec![0, 1, 2, 3]), vec![0, 1, 3]),
            2
        );
        assert_eq!(
            Solution::num_components(ListNode::from_vec(vec![0, 1, 2, 3, 4]), vec![0, 3, 1, 4]),
            2
        );
    }
}
