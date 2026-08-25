// https://leetcode.com/problems/minimum-operations-to-make-median-of-array-equal-to-k/
// 3107. Minimum Operations to Make Median of Array Equal to K
pub struct Solution;
impl Solution {
    pub fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut n = nums.len() / 2;
        match nums[n].cmp(&k) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => {
                let mut ans = 0;
                while nums[n] > k {
                    ans += (nums[n] - k) as i64;
                    if n == 0 {
                        break;
                    }
                    n -= 1;
                }
                ans
            }
            std::cmp::Ordering::Less => {
                let mut ans = 0;
                while n < nums.len() && nums[n] < k {
                    ans += (k - nums[n]) as i64;
                    n += 1;
                }
                ans
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations_to_make_median_k() {
        assert_eq!(Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4), 2);
        assert_eq!(Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 7), 3);
        assert_eq!(Solution::min_operations_to_make_median_k(vec![1, 2, 3, 4, 5, 6], 4), 0);
    }
}
