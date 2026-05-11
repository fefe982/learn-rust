// https://leetcode.com/problems/removing-minimum-and-maximum-from-array/
// 2091. Removing Minimum and Maximum From Array
pub struct Solution;
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        let mut min_pos = 0;
        let mut max_pos = 0;
        for i in 1..n {
            if nums[i] < nums[min_pos] {
                min_pos = i;
            }
            if nums[i] > nums[max_pos] {
                max_pos = i;
            }
        }
        let (min_pos, max_pos) = if min_pos < max_pos {
            (min_pos, max_pos)
        } else {
            (max_pos, min_pos)
        };
        (max_pos + 1).min(n - min_pos).min(min_pos + 1 + n - max_pos) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_deletions() {
        assert_eq!(Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]), 5);
        assert_eq!(Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]), 3);
        assert_eq!(Solution::minimum_deletions(vec![101]), 1);
    }
}
