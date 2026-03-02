// https://leetcode.com/problems/count-subarrays-with-k-distinct-integers/
// 3859. Count Subarrays With K Distinct Integers
pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32, m: i32) -> i64 {
        let cnt = |kk: i32| -> i64 {
            let mut left = 0;
            let mut cnt = vec![0; 100001];
            let mut cnt_m = 0;
            let mut ans = 0;
            let mut cnt_k = 0;
            for right in 0..nums.len() {
                let n = nums[right] as usize;
                cnt[n] += 1;
                if cnt[n] == 1 {
                    cnt_k += 1;
                }
                if cnt[n] == m {
                    cnt_m += 1;
                }
                while cnt_m >= k && cnt_k >= kk {
                    let n = nums[left] as usize;
                    cnt[n] -= 1;
                    if cnt[n] == m - 1 {
                        cnt_m -= 1;
                    }
                    if cnt[n] == 0 {
                        cnt_k -= 1;
                    }
                    left += 1;
                }
                ans += left as i64;
            }
            ans
        };
        cnt(k) - cnt(k + 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 9966], 1, 2), 1);
        assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 2, 2], 2, 2), 2);
        assert_eq!(Solution::count_subarrays(vec![3, 1, 2, 4], 2, 1), 3);
    }
}
