// https://leetcode.com/problems/find-all-good-indices/
// 2420. Find All Good Indices
pub struct Solution;
impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        for i in 1..n {
            if nums[i] <= nums[i - 1] {
                left[i] = left[i - 1] + 1;
            }
        }
        for i in (0..n - 1).rev() {
            if nums[i] <= nums[i + 1] {
                right[i] = right[i + 1] + 1;
            }
        }
        let mut ans = Vec::new();
        for i in k as usize..n - k as usize {
            if left[i - 1] >= k - 1 && right[i + 1] >= k - 1 {
                ans.push(i as i32);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_indices() {
        assert_eq!(Solution::good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2), vec![2, 3]);
        assert_eq!(Solution::good_indices(vec![2, 1, 1, 2], 2), Vec::<i32>::new());
    }
}
