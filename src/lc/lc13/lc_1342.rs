// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
// 1342. Number of Steps to Reduce a Number to Zero
pub struct Solution;
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            0
        } else {
            32 - num.leading_zeros() as i32 + num.count_ones() as i32 - 1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_steps() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
