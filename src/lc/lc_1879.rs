// https://leetcode.com/problems/minimum-xor-sum-of-two-arrays/
// 1879. Minimum XOR Sum of Two Arrays
pub struct Solution;
impl Solution {
    fn search(xor: &Vec<Vec<i32>>, i: usize, mask: usize, cache: &mut Vec<i32>) -> i32 {
        if i == xor.len() {
            return 0;
        }
        if cache[mask] >= 0 {
            return cache[mask];
        }
        let mut ans = i32::MAX;
        for j in 0..xor.len() {
            if (mask & (1 << j)) == 0 {
                ans = ans.min(Self::search(xor, i + 1, mask | (1 << j), cache) + xor[i][j]);
            }
        }
        cache[mask] = ans;
        ans
    }
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut xor = vec![vec![0; nums1.len()]; nums2.len()];
        for i in 0..nums2.len() {
            for j in 0..nums1.len() {
                xor[i][j] = nums1[j] ^ nums2[i];
            }
        }
        Self::search(&xor, 0, 0, &mut vec![-1; 1 << xor.len()])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_xor_sum() {
        assert_eq!(Solution::minimum_xor_sum(vec![1, 2], vec![2, 3]), 2);
        assert_eq!(Solution::minimum_xor_sum(vec![1, 0, 3], vec![5, 3, 4]), 8);
    }
}
