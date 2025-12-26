// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
// 2962. Count Subarrays Where Max Element Appears at Least K Times
pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let &m = nums.iter().max().unwrap();
        let mut ans = 0;
        let mut cnt = 0;
        let mut l = 0;
        let mut r = 0;
        while r < nums.len() {
            if nums[r] == m {
                cnt += 1;
                if cnt == k {
                    while cnt >= k {
                        ans += (nums.len() - r) as i64;
                        if nums[l] == m {
                            cnt -= 1;
                        }
                        l += 1;
                    }
                }
            }
            r += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }
}
