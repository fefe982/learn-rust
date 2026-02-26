// https://leetcode.com/problems/maximum-and-minimum-sums-of-at-most-size-k-subarrays/
// 3430. Maximum and Minimum Sums of At Most Size K Subarrays
pub struct Solution;
impl Solution {
    fn subarray_sum<T: Fn(i32, i32) -> bool>(nums: &Vec<i32>, k: usize, cmp: T) -> i64 {
        let mut sum = 0;
        let mut stk = std::collections::VecDeque::<(i32, usize)>::new();
        let mut psum = 0;
        for i in 0..nums.len() {
            if i >= k {
                let &(n, idx) = stk.front().unwrap();
                psum -= n as i64;
                if idx + k == i {
                    stk.pop_front();
                }
            }
            while !stk.is_empty() && cmp(nums[i], nums[stk.back().unwrap().1]) {
                let (n, idx) = stk.pop_back().unwrap();
                if let Some(&(_, idx2)) = stk.back() {
                    psum -= (n as i64) * (idx - idx2) as i64;
                } else {
                    if i >= k {
                        psum -= (n as i64) * (idx + k - i) as i64;
                    } else {
                        psum -= (n as i64) * (idx + 1) as i64;
                    }
                }
            }
            if let Some(&(_, idx2)) = stk.back() {
                psum += (nums[i] as i64) * (i - idx2) as i64;
            } else {
                psum += (nums[i] as i64) * ((i + 1).min(k)) as i64;
            }
            stk.push_back((nums[i], i));
            sum += psum;
        }
        sum
    }
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::subarray_sum(&nums, k as usize, |a, b| a > b) + Self::subarray_sum(&nums, k as usize, |a, b| a < b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_max_subarray_sum() {
        assert_eq!(Solution::min_max_subarray_sum(vec![-13, -16, -1, -19], 3), -201);
        assert_eq!(Solution::min_max_subarray_sum(vec![6, 17, 1], 3), 107);
        assert_eq!(Solution::min_max_subarray_sum(vec![1, 2, 3], 2), 20);
        assert_eq!(Solution::min_max_subarray_sum(vec![1, -3, 1], 2), -6);
    }
}
