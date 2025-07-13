// https://leetcode.cn/problems/find-numbers-with-even-number-of-digit/
// 1295. Find Numbers with Even Number of Digits
pub struct Solution;
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&x| (x >= 10 && x < 100) || (x >= 1000 && x < 10000) || (x >= 10_0000 && x < 100_0000))
            .count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_numbers() {
        assert_eq!(Solution::find_numbers(vec![100000]), 1);
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
