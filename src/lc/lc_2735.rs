// https://leetcode.com/problems/collecting-chocolates/
// 2735. Collecting Chocolates
pub struct Solution;
impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let mut f = nums.clone();
        let mut ans = nums.iter().fold(0i64, |acc, &x| acc + x as i64);
        let mut min_sum = ans;
        let n = nums.len();
        for i in 1..nums.len() {
            for j in 0..nums.len() {
                if f[j] > nums[(j + i) % n] {
                    min_sum -= (f[j] - nums[(j + i) % n]) as i64;
                    f[j] = nums[(j + i) % n];
                }
            }
            ans = ans.min(min_sum + i as i64 * x as i64);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost() {
        assert_eq!(Solution::min_cost(vec![15, 150, 56, 69, 214, 203], 42), 298);
        assert_eq!(Solution::min_cost(vec![20, 1, 15], 5), 13);
        assert_eq!(Solution::min_cost(vec![1, 2, 3], 4), 6);
    }
}
