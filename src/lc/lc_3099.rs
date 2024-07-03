// https://leetcode.cn/problems/harshad-number/
// 3099. Harshad Number
pub struct Solution;
impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut y = x;
        let mut s = 0;
        while y > 0 {
            s += y % 10;
            y /= 10;
        }
        if x % s == 0 {
            s
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_the_digits_of_harshad_number() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    }
}
