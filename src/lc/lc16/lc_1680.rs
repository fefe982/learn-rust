// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/
// 1680. Concatenation of Consecutive Binary Numbers
pub struct Solution;
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut ans = 0;
        for i in 1..=n {
            ans = ((ans << (i32::BITS - i.leading_zeros()) as i64) + i as i64) % MOD;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concatenated_binary() {
        assert_eq!(Solution::concatenated_binary(1), 1);
        assert_eq!(Solution::concatenated_binary(3), 27);
        assert_eq!(Solution::concatenated_binary(12), 505379714);
    }
}
