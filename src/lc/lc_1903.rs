// https://leetcode.com/problems/largest-odd-number-in-string/
// 1903. Largest Odd Number in String
pub struct Solution;
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let num = num.as_bytes();
        for i in (0..num.len()).rev() {
            if num[i] % 2 == 1 {
                return String::from_utf8(num[..=i].to_vec()).unwrap();
            }
        }
        "".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_odd_number_in_string() {
        assert_eq!(Solution::largest_odd_number("52".to_string()), "5".to_string());
        assert_eq!(Solution::largest_odd_number("4206".to_string()), "".to_string());
        assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427".to_string());
    }
}
