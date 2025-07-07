// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/
// 1751. Maximum Number of Events That Can Be Attended II
pub struct Solution;
impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        events.sort_unstable();
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; events.len() + 1];
        for i in (0..events.len()).rev() {
            let idx = events[i + 1..].partition_point(|v| v[0] <= events[i][1]) + i + 1;
            for j in 0..=k {
                dp[i][j] = dp[i + 1][j].max(if j == k { 0 } else { dp[idx][j + 1] + events[i][2] });
            }
        }
        *dp[0].iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_value() {
        assert_eq!(Solution::max_value(vec_vec![[1, 2, 4], [3, 4, 3], [2, 3, 1]], 2), 7);
        assert_eq!(Solution::max_value(vec_vec![[1, 2, 4], [3, 4, 3], [2, 3, 10]], 2), 10);
        assert_eq!(
            Solution::max_value(vec_vec![[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]], 3),
            9
        );
    }
}
