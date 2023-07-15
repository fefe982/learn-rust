// https://leetcode.com/problems/split-array-largest-sum/
// 410. Split Array Largest Sum
pub struct Solution;
impl Solution {
    fn check(nums: &Vec<i32>, s: i32, k: i32) -> bool {
        let mut sum = 0;
        let mut cnt = 1;
        for &n in nums {
            if sum + n > s {
                sum = n;
                cnt += 1;
            } else {
                sum += n;
            }
        }
        cnt <= k
    }
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut low = 0;
        let mut high = 0;
        for &n in &nums {
            low = low.max(n);
            high += n;
        }
        if k == 1 {
            return high;
        }
        if Self::check(&nums, low, k) {
            return low;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&nums, mid, k) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_array() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
    }
}
