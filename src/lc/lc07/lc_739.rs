// https://leetcode.com/problems/daily-temperatures/
// 739. Daily Temperatures
pub struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];
        let mut stack = Vec::new();
        for (i, t) in temperatures.into_iter().enumerate().rev() {
            while let Some(&(ni, nt)) = stack.last() {
                if t < nt {
                    ans[i] = ni as i32 - i as i32;
                    break;
                }
                stack.pop();
            }
            stack.push((i, t));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(Solution::daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(Solution::daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
