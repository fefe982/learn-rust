// https://leetcode.com/problems/find-the-number-of-possible-ways-for-an-event/
// 3317. Find the Number of Possible Ways for an Event
pub struct Solution;
impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        let m = 1_000_000_007i64;
        let mut dp = vec![0; x as usize + 1];
        dp[1] = 1;
        for _ in 2..=n {
            let mut ndp = vec![0; x as usize + 1];
            for s in 1..=x as usize {
                ndp[s] = (dp[s] * s as i64 + dp[s - 1]) % m;
            }
            dp = ndp;
        }
        let mut res = 0;
        let mut fx = 1;
        let mut py = 1;
        for i in 1..=x as usize {
            fx = (fx * (x as i64 + 1 - i as i64)) % m;
            py = (py * y as i64) % m;
            res = (res + (fx * dp[i]) % m * py) % m;
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_ways() {
        assert_eq!(Solution::number_of_ways(1, 2, 3), 6);
        assert_eq!(Solution::number_of_ways(5, 2, 1), 32);
        assert_eq!(Solution::number_of_ways(3, 3, 4), 684);
    }
}
