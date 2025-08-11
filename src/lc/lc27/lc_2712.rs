// https://leetcode.cn/problems/minimum-cost-to-make-all-characters-equal/
// 2712. Minimum Cost to Make All Characters Equal
pub struct Solution;
impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let mut sum = 0;
        let s = s.as_bytes();
        let n = s.len();
        for i in 1..=n / 2 {
            if s[i] != s[i - 1] {
                sum += i as i64;
            }
        }
        for i in n / 2 + 1..n {
            if s[i] != s[i - 1] {
                sum += (n - i) as i64;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(Solution::minimum_cost("0011".to_string()), 2);
        assert_eq!(Solution::minimum_cost("010101".to_string()), 9);
    }
}
