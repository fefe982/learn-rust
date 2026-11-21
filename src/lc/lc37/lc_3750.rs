// https://leetcode.com/problems/minimum-number-of-flips-to-reverse-binary-string/
// 3750. Minimum Number of Flips to Reverse Binary String
pub struct Solution;
impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let s = format!("{:b}", n);
        let b = s.as_bytes();
        let mut ans = 0;
        for i in 0..b.len() / 2 {
            if b[i] != b[b.len() - 1 - i] {
                ans += 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_flips() {
        assert_eq!(Solution::minimum_flips(7), 0);
        assert_eq!(Solution::minimum_flips(10), 4);
    }
}
