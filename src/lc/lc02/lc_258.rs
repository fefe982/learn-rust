// https://leetcode.com/problems/add-digits/
// 258. Add Digits
pub struct Solution;
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_digits() {
        assert_eq!(Solution::add_digits(38), 2);
        assert_eq!(Solution::add_digits(0), 0);
    }
}
