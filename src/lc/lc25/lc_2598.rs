// https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/
// 2598. Smallest Missing Non-negative Integer After Operations
pub struct Solution;
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut v = vec![-1; value as usize];
        for n in nums {
            v[((n % value + value) % value) as usize] += 1;
        }
        let mut min = i32::MAX;
        let mut val = 0;
        for (i, n) in v.into_iter().enumerate() {
            if n < min {
                min = n;
                val = (n + 1) * value + i as i32;
            }
        }
        val
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_smallest_integer() {
        assert_eq!(Solution::find_smallest_integer(vec![1, 3, 5, 7], 2), 0);
        assert_eq!(Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5), 4);
        assert_eq!(Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5), 4);
        assert_eq!(Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7), 2);
    }
}
