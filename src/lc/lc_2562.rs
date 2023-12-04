// https://leetcode.com/problems/find-the-array-concatenation-value/
// 2562. Find the Array Concatenation Value
pub struct Solution;
impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut sum = 0i64;
        let l = nums.len() - 1;
        for i in 0..nums.len() / 2 {
            let mut pow = 1i64;
            while pow <= nums[l - i] as i64 {
                pow *= 10;
            }
            sum += nums[i] as i64 * pow + nums[l - i] as i64;
        }
        if l % 2 == 0 {
            sum += nums[l / 2] as i64;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_the_array_conc_val() {
        assert_eq!(
            Solution::find_the_array_conc_val(vec![
                1, 78, 27, 48, 14, 86, 79, 68, 77, 20, 57, 21, 18, 67, 5, 51, 70, 85, 47, 56, 22, 79, 41, 8, 39, 81,
                59, 74, 14, 45, 49, 15, 10, 28
            ]),
            74322
        );
        assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
        assert_eq!(Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]), 673);
    }
}
