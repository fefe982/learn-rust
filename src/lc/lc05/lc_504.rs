// https://leetcode.com/problems/base-7/
// 504. Base 7
pub struct Solution;
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut minus = false;
        let mut num = num;
        if num < 0 {
            minus = true;
            num = -num;
        }
        let mut res = String::new();
        if num == 0 {
            return "0".to_string();
        }
        while num > 0 {
            res.push_str(&(num % 7).to_string());
            num /= 7;
        }
        if minus {
            res.push('-');
        }
        res.chars().rev().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_to_base7() {
        assert_eq!(Solution::convert_to_base7(100), "202");
        assert_eq!(Solution::convert_to_base7(-7), "-10");
    }
}
