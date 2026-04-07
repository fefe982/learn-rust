// https://leetcode.com/problems/minimum-distance-to-the-target-element/
// 1848. Minimum Distance to the Target Element
pub struct Solution;
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut ans = i32::MAX;
        for (i, n) in nums.into_iter().enumerate() {
            if n == target {
                ans = ans.min((i as i32 - start).abs());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_min_distance() {
        assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
        assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
        assert_eq!(Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0), 0);
    }
}
