// https://leetcode.com/problems/remove-trailing-zeros-from-a-string/
// 2710. Remove Trailing Zeros From a String
pub struct Solution;
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut num = num;
        while let Some(c) = num.pop() {
            if c != '0' {
                num.push(c);
                break;
            }
        }
        num
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_trailing_zeros() {
        assert_eq!(Solution::remove_trailing_zeros("51230100".to_string()), "512301");
        assert_eq!(Solution::remove_trailing_zeros("123".to_string()), "123");
    }
}
