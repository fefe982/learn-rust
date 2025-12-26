// https://leetcode.com/problems/find-the-k-or-of-an-array/
// 2917. Find the K-or of an Array
pub struct Solution;
impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = vec![0; k as usize + 1];
        ans[0] = !0;
        for num in nums {
            for i in (1..=k as usize).rev() {
                ans[i] |= ans[i - 1] & num;
            }
        }
        ans[k as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_k_or() {
        assert_eq!(Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
        assert_eq!(Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
        assert_eq!(Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
}
