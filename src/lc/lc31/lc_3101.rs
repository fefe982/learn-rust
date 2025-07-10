// https://leetcode.com/problems/count-alternating-subarrays/
// 3101. Count Alternating Subarrays
pub struct Solution;
impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut last = -1;
        let mut len = 0;
        for n in nums {
            if n != last {
                len += 1;
            } else {
                len = 1;
            }
            res += len as i64;
            last = n;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_alternating_subarrays() {
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
    }
}
