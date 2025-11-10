// https://leetcode.com/problems/count-subarrays-with-majority-element-ii/
// 3793. Count Subarrays With Majority Element II
pub struct Solution;
impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let len = nums.len();
        let idx = |x: i32| (x + len as i32 + 1) as usize;
        let mut cnt = vec![0; 2 * len + 2];
        cnt[idx(0)] = 1;
        let mut s = 0;
        let mut ss = 0;
        let mut ans = 0;
        for n in nums {
            if n == target {
                ss += cnt[idx(s)];
                s += 1;
            } else {
                s -= 1;
                ss -= cnt[idx(s)];
            }
            cnt[idx(s)] += 1;
            ans += ss as i64;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_majority_subarrays() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }
}
