// https://leetcode.com/problems/number-of-sets-of-k-non-overlapping-line-segments/
// 1621. Number of Sets of K Non-Overlapping Line Segments
pub struct Solution;
impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let t = MOD / b + 1;
                a = a * t % MOD;
                b = b * t % MOD;
            }
            a / b
        };
        let nn = (n + k) as i64;
        let kk = k as i64 * 2;
        let mut ans = 1;
        for i in 1..=kk {
            ans = div(ans * (nn - i) % MOD, i);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_sets() {
        assert_eq!(Solution::number_of_sets(4, 2), 5);
        assert_eq!(Solution::number_of_sets(3, 1), 3);
        assert_eq!(Solution::number_of_sets(30, 7), 796297179);
    }
}
