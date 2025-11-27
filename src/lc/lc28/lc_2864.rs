// https://leetcode.com/problems/maximum-odd-binary-number/
// 2864. Maximum Odd Binary Number
pub struct Solution;
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut n1 = 0;
        let mut n0 = 0;
        for c in s.chars() {
            if c == '1' {
                n1 += 1;
            } else {
                n0 += 1;
            }
        }
        n1 -= 1;
        let mut ans = "".to_owned();
        ans.push_str(&"1".repeat(n1));
        ans.push_str(&"0".repeat(n0));
        ans.push_str("1");
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_odd_binary_number() {
        assert_eq!(Solution::maximum_odd_binary_number("010".to_string()), "001");
        assert_eq!(Solution::maximum_odd_binary_number("0101".to_string()), "1001");
    }
}
