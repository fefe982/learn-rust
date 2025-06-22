// https://leetcode.com/problems/next-permutation/
// 31. Next Permutation
pub struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n == 1 {
            return;
        }
        let mut i = n - 1;
        while i >= 1 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i >= 1 {
            let mut j = n - 1;
            while j > i - 1 && nums[j] <= nums[i - 1] {
                j -= 1;
            }
            nums.swap(i - 1, j);
        }
        let mut l = i;
        let mut r = n - 1;
        while l < r {
            nums.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(mut nums: Vec<i32>, expect: Vec<i32>) {
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, expect);
    }
    #[test]
    fn test_next_permutation() {
        check(vec![1, 2, 3], vec![1, 3, 2]);
        check(vec![3, 2, 1], vec![1, 2, 3]);
        check(vec![1, 1, 5], vec![1, 5, 1]);
        check(vec![1], vec![1]);
    }
}
