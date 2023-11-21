// https://leetcode.com/problems/count-nice-pairs-in-an-array/
// 1814. Count Nice Pairs in an Array
pub struct Solution;
impl Solution {
    fn rev(x: i32) -> i64 {
        let mut r = 0;
        let mut xx = x as i64;
        while xx > 0 {
            r = r * 10 + xx % 10;
            xx /= 10;
        }
        x as i64 - r
    }
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        (nums
            .into_iter()
            .fold(std::collections::HashMap::<i64, i32>::new(), |mut acc, x| {
                *acc.entry(Self::rev(x)).or_default() += 1;
                acc
            })
            .into_iter()
            .fold(0, |acc, (_, c)| acc + c as i64 * (c as i64 - 1) / 2)
            % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_nice_pairs() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
