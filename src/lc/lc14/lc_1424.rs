// https://leetcode.com/problems/diagonal-traverse-ii/
// 1424. Diagonal Traverse II
pub struct Solution;
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let l = nums.len();
        let mut next = vec![usize::MAX; nums.len() + 1];
        let mut idx = vec![0; nums.len()];
        let mut ret = vec![];
        let mut head = 1;
        next[l] = 0;
        while next[l] < l {
            let mut cur = next[l];
            let mut pre = l;
            while cur < l {
                ret.push(nums[cur][idx[cur]]);
                idx[cur] += 1;
                if idx[cur] >= nums[cur].len() {
                    next[pre] = next[cur];
                } else {
                    pre = cur;
                }
                cur = next[cur];
            }
            if head != l {
                next[head] = next[l];
                next[l] = head;
                head += 1;
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_diagonal_order() {
        assert_eq!(
            Solution::find_diagonal_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec_vec![
                [1, 2, 3, 4, 5],
                [6, 7],
                [8],
                [9, 10, 11],
                [12, 13, 14, 15, 16]
            ]),
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        )
    }
}
