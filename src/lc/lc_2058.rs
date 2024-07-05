// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/
// 2058. Find the Minimum and Maximum Number of Nodes Between Critical Points
pub struct Solution;
use super::linked_list::ListNode;
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut pos = 0;
        let mut pos_first = 0;
        let mut pos_last = 0;
        let mut min_dist = i32::MAX;
        let mut head = head;
        let mut val = vec![0; 3];
        while let Some(node) = head {
            pos += 1;
            let v = node.val;
            val[0] = val[1];
            val[1] = val[2];
            val[2] = v;
            head = node.next;
            if val[0] == 0 {
                continue;
            }
            if (val[1] > val[0] && val[1] > val[2]) || (val[1] < val[0] && val[1] < val[2]) {
                if pos_first == 0 {
                    pos_first = pos;
                }
                if pos_last > 0 {
                    min_dist = min_dist.min(pos - pos_last);
                }
                pos_last = pos;
            }
        }
        if min_dist == i32::MAX {
            vec![-1, -1]
        } else {
            vec![min_dist, pos_last - pos_first]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nodes_between_critical_points() {
        assert_eq!(
            Solution::nodes_between_critical_points(ListNode::from_vec(vec![3, 1])),
            [-1, -1]
        );
        assert_eq!(
            Solution::nodes_between_critical_points(ListNode::from_vec(vec![5, 3, 1, 2, 5, 1, 2])),
            [1, 3]
        );
    }
}
