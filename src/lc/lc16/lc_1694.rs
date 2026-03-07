// https://leetcode.com/problems/reformat-phone-number/
// 1694. Reformat Phone Number
pub struct Solution;
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut number = number.replace("-", "");
        number = number.replace(" ", "");
        let mut res = String::new();
        let mut i = 0;
        while i < number.len() {
            if number.len() - i > 4 {
                res.push_str(&number[i..i + 3]);
                res.push('-');
                i += 3;
            } else if number.len() - i == 4 {
                res.push_str(&number[i..i + 2]);
                res.push('-');
                res.push_str(&number[i + 2..]);
                break;
            } else {
                res.push_str(&number[i..]);
                break;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reformat_number() {
        assert_eq!(Solution::reformat_number("1-23-45 6".to_string()), "123-456");
        assert_eq!(Solution::reformat_number("123 4-567".to_string()), "123-45-67");
        assert_eq!(Solution::reformat_number("123 4-5678".to_string()), "123-456-78");
    }
}
