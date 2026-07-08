// https://leetcode.com/problems/path-existence-queries-in-a-graph-i/
// 3532. Path Existence Queries in a Graph I
pub struct Solution;
impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut nums = nums;
        let mut last = -max_diff - 1;
        let mut n = 0;
        for i in 0..nums.len() {
            if nums[i] - last > max_diff {
                n += 1;
            }
            last = nums[i];
            nums[i] = n;
        }
        queries
            .into_iter()
            .map(|q| nums[q[0] as usize] == nums[q[1] as usize])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn path_existence_queries() {
        assert_eq!(
            Solution::path_existence_queries(2, vec![1, 3], 1, vec_vec![[0, 0], [0, 1]]),
            vec![true, false]
        );
        assert_eq!(
            Solution::path_existence_queries(5, vec![2, 5, 6, 8], 2, vec_vec![[0, 1], [0, 2], [1, 3], [2, 3]]),
            vec![false, false, true, true]
        );
    }
}
