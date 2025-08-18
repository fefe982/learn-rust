// https://leetcode.com/problems/binary-prefix-divisible-by-5/
// 1018. Binary Prefix Divisible By 5
pub struct Solution;
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut ans = Vec::with_capacity(nums.len());
        let mut sum = 0;
        for num in nums {
            sum = (sum * 2 + num) % 5;
            ans.push(sum == 0);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefixes_div_by5() {
        assert_eq!(Solution::prefixes_div_by5(vec![0, 1, 1]), [true, false, false]);
        assert_eq!(Solution::prefixes_div_by5(vec![1, 1, 1]), [false, false, false]);
    }
}
