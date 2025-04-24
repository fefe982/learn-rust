// https://leetcode.com/problems/count-of-interesting-subarrays/
// 2845. Count of Interesting Subarrays
pub struct Solution;
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut cnt = vec![0; (modulo as usize).min(nums.len() + 1)];
        let mut ans = 0;
        cnt[0] = 1;
        let mut s = 0;
        for n in nums {
            if n % modulo == k {
                s += 1;
            }
            ans += cnt.get(((s + modulo - k) % modulo) as usize).unwrap_or(&0);
            cnt[(s % modulo) as usize] += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_interesting_subarrays() {
        assert_eq!(Solution::count_interesting_subarrays(vec![2, 4], 7, 2), 0);
        assert_eq!(Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1), 3);
        assert_eq!(Solution::count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0), 2);
    }
}
