// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/
// 1760. Minimum Limit of Balls in a Bag
pub struct Solution;
impl Solution {
    fn check(nums: &Vec<i32>, max_operations: i32, limit: i32) -> bool {
        let mut total = 0;
        for &n in nums {
            total += (n - 1) / limit;
            if total > max_operations {
                return false;
            }
        }
        true
    }
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        if Self::check(&nums, max_operations, 1) {
            return 1;
        }
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();
        while l + 1 < r {
            let mid = (l + r) / 2;
            if Self::check(&nums, max_operations, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_size() {
        assert_eq!(Solution::minimum_size(vec![9], 2), 3);
        assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
    }
}
