// https://leetcode.com/problems/house-robber-iv/
// 2560. House Robber IV
pub struct Solution;
impl Solution {
    fn check(nums: &Vec<i32>, n: i32, k: i32) -> bool {
        let mut c = 0;
        let mut v = false;
        for &num in nums {
            if !v && num <= n {
                c += 1;
                v = true;
            } else {
                v = false;
            }
        }
        c >= k
    }
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &n in &nums {
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
        }
        if Self::check(&nums, min, k) {
            return min;
        }
        while min + 1 < max {
            let mid = (min + max) / 2;
            if Self::check(&nums, mid, k) {
                max = mid;
            } else {
                min = mid;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_capability() {
        assert_eq!(Solution::min_capability(vec![2, 3, 5, 9], 2), 5);
        assert_eq!(Solution::min_capability(vec![2, 7, 9, 3, 1], 2), 2);
    }
}
