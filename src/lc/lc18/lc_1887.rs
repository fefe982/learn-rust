// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/
// 1887. Reduction Operations to Make the Array Elements Equal
pub struct Solution;
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut last = i32::MAX;
        let mut count = 0;
        let mut sum = 0;
        for n in nums.into_iter().rev() {
            if n != last {
                sum += count;
            }
            count += 1;
            last = n;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reduction_operations() {
        assert_eq!(Solution::reduction_operations(vec![5, 1, 3]), 3);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 1]), 0);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 2, 2, 3]), 4);
    }
}
