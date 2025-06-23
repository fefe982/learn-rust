// https://leetcode.com/problems/number-of-ways-to-split-array/
// 2270. Number of Ways to Split Array
pub struct Solution;
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut diff = -nums.iter().map(|i| *i as i64).sum::<i64>();
        let mut res = 0;
        for i in 0..nums.len() - 1 {
            diff += nums[i] as i64 * 2;
            if diff >= 0 {
                res += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ways_to_split_array() {
        assert_eq!(Solution::ways_to_split_array(vec![10, 4, -8, 7]), 2);
        assert_eq!(Solution::ways_to_split_array(vec![2, 3, 1, 0]), 2);
    }
}
