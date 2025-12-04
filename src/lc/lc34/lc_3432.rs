// https://leetcode.com/problems/count-partitions-with-even-sum-difference/
// 3432. Count Partitions With Even Sum Difference
pub struct Solution;
impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if nums.into_iter().sum::<i32>() % 2 == 0 {
            n - 1
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_partitions() {
        assert_eq!(Solution::count_partitions(vec![10, 10, 3, 7, 6]), 4);
        assert_eq!(Solution::count_partitions(vec![1, 2, 2]), 0);
        assert_eq!(Solution::count_partitions(vec![2, 4, 6, 8]), 3);
    }
}
