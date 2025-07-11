// https://leetcode.com/problems/maximum-value-of-a-string-in-an-array/
// 2496. Maximum Value of a String in an Array
pub struct Solution;
impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter().fold(0, |m, x| {
            if let Ok(n) = x.parse::<i32>() {
                n
            } else {
                x.len() as i32
            }
            .max(m)
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_value() {
        assert_eq!(
            Solution::maximum_value(vec_str!["alic3", "bob", "3", "4", "00000"]),
            5
        );
        assert_eq!(
            Solution::maximum_value(vec_str!["1", "01", "001", "0001"]),
            1
        );
    }
}
