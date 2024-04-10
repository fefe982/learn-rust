// https://leetcode.com/problems/maximum-binary-string-after-change/
// 1702. Maximum Binary String After Change
pub struct Solution;
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut n11 = 0;
        let mut n0 = 0;
        let mut n12 = 0;
        for c in binary.chars() {
            if c == '1' {
                if n0 == 0 {
                    n11 += 1;
                } else {
                    n12 += 1;
                }
            } else if c == '0' {
                if n0 == 0 {
                    n0 += 1;
                } else {
                    n11 += 1;
                }
            }
        }
        if n0 == 0 {
            "1".repeat(n11)
        } else {
            "1".repeat(n11) + "0" + &"1".repeat(n12)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_binary_string() {
        assert_eq!(Solution::maximum_binary_string("000110".to_string()), "111011");
        assert_eq!(Solution::maximum_binary_string("01".to_string()), "01");
    }
}
