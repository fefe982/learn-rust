// https://leetcode.com/problems/maximum-value-after-insertion/
// 1881. Maximum Value after Insertion
pub struct Solution;
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let x_digit = (b'0' + x as u8) as char;
        let bytes = n.as_bytes();
        let mut ans = String::with_capacity(n.len() + 1);
        if bytes[0] == b'-' {
            // For negative numbers, insert before first digit greater than x.
            for i in 1..bytes.len() {
                if bytes[i] > b'0' + x as u8 {
                    ans.push_str(&n[..i]);
                    ans.push(x_digit);
                    ans.push_str(&n[i..]);
                    return ans;
                }
            }
            ans.push_str(&n);
            ans.push(x_digit);
            ans
        } else {
            // For positive numbers, insert before first digit smaller than x.
            for i in 0..bytes.len() {
                if bytes[i] < b'0' + x as u8 {
                    ans.push_str(&n[..i]);
                    ans.push(x_digit);
                    ans.push_str(&n[i..]);
                    return ans;
                }
            }
            ans.push_str(&n);
            ans.push(x_digit);
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_value() {
        assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
        assert_eq!(Solution::max_value("-13".to_string(), 2), "-123".to_string());
    }
}
