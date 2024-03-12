// https://leetcode.com/problems/allocate-mailboxes/
// 1478. Allocate Mailboxes
pub struct Solution;
impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let mut houses = houses;
        houses.sort_unstable();
        let n = houses.len();
        let k = k as usize;
        let mut sum = vec![0; n + 1];
        for i in 1..=n {
            sum[i] = sum[i - 1] + houses[i - 1];
        }
        let mut dp = vec![1000000; n + 1];
        dp[0] = 0;
        for ik in 1..=k {
            for i in (ik - 1..n).rev() {
                for s in ik - 1..=i {
                    let mid1 = (s + i) / 2;
                    let mid2 = (s + i + 1) / 2;
                    let add = sum[i + 1] - sum[mid2] - sum[mid1 + 1] + sum[s];
                    dp[i + 1] = dp[i + 1].min(dp[s] + add);
                }
            }
        }
        dp[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance(vec![1, 4, 8, 10, 20], 3), 5);
        assert_eq!(Solution::min_distance(vec![2, 3, 5, 12, 18], 2), 9);
    }
}
