// https://leetcode.cn/problems/6CE719/
// LCP 61. 气温变化趋势
pub struct Solution;
impl Solution {
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        let mut con = 0;
        let mut ans = 0;
        for i in 1..temperature_a.len() {
            if temperature_a[i].cmp(&temperature_a[i - 1]) == temperature_b[i].cmp(&temperature_b[i - 1]) {
                con += 1;
            } else {
                ans = ans.max(con);
                con = 0;
            }
        }
        ans.max(con)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_temperature_trend() {
        assert_eq!(
            Solution::temperature_trend(vec![21, 18, 18, 18, 31], vec![34, 32, 16, 16, 17]),
            2
        );
        assert_eq!(
            Solution::temperature_trend(vec![5, 10, 16, -6, 15, 11, 3], vec![16, 22, 23, 23, 25, 3, -16]),
            3
        );
    }
}
