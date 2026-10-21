// https://leetcode.com/problems/minimum-operations-to-sort-a-permutation/
// 3942. Minimum Operations to Sort a Permutation
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut iz = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                iz = i;
                break;
            }
        }
        let mut min = i32::MAX;
        let mut fwd = true;
        for i in 1..nums.len() {
            if nums[(iz + i) % nums.len()] != i as i32 {
                fwd = false;
                break;
            }
        }
        if fwd {
            min = min.min(iz as i32).min((nums.len() - iz + 2) as i32);
        }
        let mut bwd = true;
        for i in 1..nums.len() {
            if nums[(iz + nums.len() - i) % nums.len()] != i as i32 {
                bwd = false;
                break;
            }
        }
        if bwd {
            min = min
                .min((nums.len() - iz) as i32)
                .min(((iz + 1) % nums.len()) as i32 + 1);
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![0, 2, 1]), 2);
        assert_eq!(Solution::min_operations(vec![1, 0, 2]), 2);
        assert_eq!(Solution::min_operations(vec![2, 0, 1, 3]), -1);
    }
}
