// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
// 1685. Sum of Absolute Differences in a Sorted Array
pub struct Solution;
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = vec![0];
        let mut nums = nums.iter().cloned().zip(0usize..).collect::<Vec<_>>();
        nums.sort();
        nums.iter().fold(0, |acc, &(n, _)| {
            sum.push(acc + n);
            acc + n
        });
        let mut res = vec![0; nums.len()];
        let len = nums.len();
        for (idx, (n, i)) in nums.into_iter().enumerate() {
            res[i] = n * (idx as i32 - 0) - sum[idx] + sum[len] - sum[idx + 1] - n * (len as i32 - idx as i32 - 1);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_sum_absolute_differences() {
        assert_eq!(Solution::get_sum_absolute_differences(vec![2, 3, 5]), vec![4, 3, 5]);
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
            vec![24, 15, 13, 15, 21]
        );
    }
}
