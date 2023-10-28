// https://leetcode.cn/problems/maximum-sum-circular-subarray/
// 918. Maximum Sum Circular Subarray
pub struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut sums: Vec<(usize, i32)> = vec![];
        let mut s = 0;
        for (i, &n) in nums.iter().enumerate() {
            s += n;
            while let Some((j, ls)) = sums.pop() {
                if ls > s {
                    sums.push((j, ls));
                    break;
                }
            }
            sums.push((i, s));
        }
        let mut sl = 0;
        let mut sumi = 0;
        let mut max = i32::MIN;
        for (&n, i) in nums.iter().zip(nums.len()..) {
            while sums[sumi].0 + nums.len() < i {
                sumi += 1;
            }
            max = max.max(sums[sumi].1 - sl);
            sl += n;
            s += n;
            while let Some((j, ls)) = sums.pop() {
                if ls > s {
                    sums.push((j, ls));
                    break;
                }
            }
            sums.push((i, s));
            if sums.len() <= sumi {
                sumi = sums.len() - 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_subarray_sum_circular() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }
}
