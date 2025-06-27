// https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing/
// 801. Minimum Swaps To Make Sequences Increasing
pub struct Solution;
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ln1 = nums1[0];
        let mut ln2 = nums2[0];
        let mut swap0 = 0;
        let mut swap1 = 1;
        for (&n1, &n2) in nums1.iter().zip(nums2.iter()).skip(1) {
            let mut nswap0 = i32::MAX;
            let mut nswap1 = i32::MAX;
            if n1 > ln1 && n2 > ln2 {
                nswap0 = nswap0.min(swap0);
                nswap1 = nswap1.min(swap1 + 1);
            }
            if n1 > ln2 && n2 > ln1 {
                nswap0 = nswap0.min(swap1);
                nswap1 = nswap1.min(swap0 + 1);
            }
            swap0 = nswap0;
            swap1 = nswap1;
            ln1 = n1;
            ln2 = n2;
        }
        swap0.min(swap1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_swap() {
        assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
        assert_eq!(Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]), 1);
    }
}
