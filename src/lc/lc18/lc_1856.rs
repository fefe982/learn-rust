// https://leetcode.com/problems/maximum-subarray-min-product/
// 1856. Maximum Subarray Min-Product
pub struct Solution;
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        let mut prefix = vec![0_i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let mut left = vec![-1_i32; n];
        let mut right = vec![n as i32; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        // Previous strictly smaller element index.
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if nums[top] >= nums[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                left[i] = top as i32;
            }
            stack.push(i);
        }

        stack.clear();
        // Next smaller or equal element index.
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if nums[top] > nums[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                right[i] = top as i32;
            }
            stack.push(i);
        }

        let mut best = 0_i64;
        for i in 0..n {
            let l = (left[i] + 1) as usize;
            let r = right[i] as usize;
            let sum = prefix[r] - prefix[l];
            let prod = sum * nums[i] as i64;
            if prod > best {
                best = prod;
            }
        }

        (best % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sum_min_product() {
        assert_eq!(Solution::max_sum_min_product(vec![1, 2, 3, 2]), 14);
        assert_eq!(Solution::max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
        assert_eq!(Solution::max_sum_min_product(vec![3, 1, 5, 6, 4, 2]), 60);
    }
}
