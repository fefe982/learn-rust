// https://leetcode.com/problems/number-of-ways-to-split-a-string/
// 1573. Number of Ways to Split a String
pub struct Solution;
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let s = s.as_bytes();
        let ones = s.iter().fold(0, |acc, &x| acc + (x == b'1') as i32);
        const MOD: i64 = 1_000_000_007;
        if ones % 3 != 0 {
            return 0;
        }
        if ones == 0 {
            let n = s.len() as i64;
            return ((n - 1) * (n - 2) / 2 % MOD) as i32;
        }
        let i1 = ones / 3;
        let i2 = i1 * 2;
        let mut cnt = 0;
        let mut c1 = 0;
        let mut c2 = 0;
        for i in 0..s.len() {
            if s[i] == b'1' {
                cnt += 1;

                if cnt == i1 {
                    c1 = i;
                }
                if cnt == i1 + 1 {
                    c1 = i - c1;
                }
                if cnt == i2 {
                    c2 = i;
                }
                if cnt == i2 + 1 {
                    c2 = i - c2;
                }
            }
        }
        ((c1 * c2) as i64 % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_ways() {
        assert_eq!(Solution::num_ways("10101".to_string()), 4);
        assert_eq!(Solution::num_ways("1001".to_string()), 0);
        assert_eq!(Solution::num_ways("0000".to_string()), 3);
    }
}
