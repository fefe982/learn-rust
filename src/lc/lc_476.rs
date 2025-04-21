// https://leetcode.com/problems/number-complement/
// 476. Number Complement
pub struct Solution;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        num ^ ((1 << (32 - num.leading_zeros())) - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_complement() {
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement(1), 0);
    }
}
