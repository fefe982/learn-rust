// https://leetcode.com/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/
// 2400. Number of Ways to Reach a Position After Exactly k Steps
pub struct Solution;
impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        let d = (end_pos - start_pos).abs() as i64;
        let k = k as i64;
        if d > k || (d & 1) != (k & 1) {
            return 0;
        }
        let low = (k - d) / 2;
        let mut ans = 1;
        const MOD: i64 = 1_000_000_007;
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let t = MOD / b;
                a = a * (t + 1) % MOD;
                b = b * (t + 1) % MOD;
            }
            a / b
        };
        for i in 1..=low {
            ans = (ans * (k + 1 - i)) % MOD;
            ans = div(ans, i);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_ways() {
        assert_eq!(Solution::number_of_ways(1, 2, 3), 3);
        assert_eq!(Solution::number_of_ways(2, 5, 10), 0);
    }
}
