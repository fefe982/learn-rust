// https://leetcode.com/problems/spiral-matrix-iv/
// 2326. Spiral Matrix IV
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![-1; n as usize]; m as usize];
        let mut top = 0;
        let mut bottom = matrix.len();
        let mut left = 0;
        let mut right = matrix[0].len();
        let mut head = head;
        'lp: loop {
            for i in left..right {
                if head.is_none() {
                    break 'lp;
                }
                let node = head.unwrap();
                matrix[top][i] = node.val;
                head = node.next;
            }
            top += 1;
            if bottom <= top {
                break;
            }
            for i in top..bottom {
                if head.is_none() {
                    break 'lp;
                }
                let node = head.unwrap();
                matrix[i][right - 1] = node.val;
                head = node.next;
            }
            right -= 1;
            if right <= left {
                break;
            }
            for i in (left..right).rev() {
                if head.is_none() {
                    break 'lp;
                }
                let node = head.unwrap();
                matrix[bottom - 1][i] = node.val;
                head = node.next;
            }
            bottom -= 1;
            if bottom <= top {
                break;
            }
            for i in (top..bottom).rev() {
                if head.is_none() {
                    break 'lp;
                }
                let node = head.unwrap();
                matrix[i][left] = node.val;
                head = node.next;
            }
            left += 1;
            if right <= left {
                break;
            }
        }
        matrix
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn spiral_matrix() {
        assert_eq!(
            Solution::spiral_matrix(3, 5, ListNode::from_vec(vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0])),
            vec_vec![[3, 0, 2, 6, 8], [5, 0, -1, -1, 1], [5, 2, 4, 9, 7]]
        );
        assert_eq!(
            Solution::spiral_matrix(1, 4, ListNode::from_vec(vec![0, 1, 2])),
            vec_vec![[0, 1, 2, -1]]
        );
    }
}
