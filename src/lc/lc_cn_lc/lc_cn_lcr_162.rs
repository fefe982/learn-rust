// https://leetcode.cn/problems/1nzheng-shu-zhong-1chu-xian-de-ci-shu-lcof/
// LCR 162. 数字 1 的个数
pub struct Solution;
impl Solution {
    pub fn digit_one_in_number(num: i32) -> i32 {
        super::super::lc02::lc_233::Solution::count_digit_one(num)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::digit_one_in_number(0), 0);
        assert_eq!(Solution::digit_one_in_number(13), 6);
    }
}
