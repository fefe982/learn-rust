// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
// 698. Partition to K Equal Sum Subsets
pub struct Solution;
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        let mx = *nums.iter().max().unwrap();
        if mx > sum / k {
            return false;
        }
        let n = nums.len();
        let seg = sum / k;
        let mut dp = vec![false; 1 << n];
        let mut sum = vec![0; 1 << n];
        dp[0] = true;
        for i in 1..1 << n {
            let mut j: usize = i;
            while j > 0 {
                let bit = j.trailing_zeros() as usize;
                let ni = i & !(1 << bit);
                if dp[ni] && (nums[bit] % seg == 0 || (sum[ni] + nums[bit] <= seg)) {
                    dp[i] = true;
                    sum[i] = (sum[ni] + nums[bit]) % seg;
                    break;
                }
                j = (j - 1) & j;
            }
        }
        dp[(1 << n) - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_partition_k_subsets() {
        assert_eq!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
        assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
    }
}
