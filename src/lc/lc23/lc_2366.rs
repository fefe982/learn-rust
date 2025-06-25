// https://leetcode.com/problems/minimum-replacements-to-sort-the-array/
// 2366. Minimum Replacements to Sort the Array
pub struct Solution;
impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut sum = 0i64;
        let mut bound = i32::MAX;
        for n in nums.into_iter().rev() {
            let mut split = n / bound;
            if n % bound != 0 {
                split += 1;
            }
            bound = n / split;
            sum += split as i64 - 1;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_replacement() {
        assert_eq!(Solution::minimum_replacement(vec![3, 9, 3]), 2);
        assert_eq!(Solution::minimum_replacement(vec![1, 2, 3, 4, 5]), 0);
    }
}
