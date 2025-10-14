// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/
// 3350. Adjacent Increasing Subarrays Detection II
pub struct Solution;
impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut m = 1;
        let mut mp = 0;
        let mut cnt = 1;
        let mut lcnt = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                cnt += 1;
                m = m.max(cnt);
                mp = mp.max(cnt.min(lcnt));
            } else {
                lcnt = cnt;
                cnt = 1;
                mp = mp.max(1);
            }
        }
        mp.max(m / 2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_increasing_subarrays() {
        assert_eq!(Solution::max_increasing_subarrays(vec![19, 5]), 1);
        assert_eq!(
            Solution::max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1]),
            3
        );
        assert_eq!(
            Solution::max_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7]),
            2
        );
    }
}
