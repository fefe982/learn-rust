// https://leetcode.com/problems/find-the-maximum-sequence-value-of-array/
// 3287. Find the Maximum Sequence Value of Array
pub struct Solution;
impl Solution {
    fn seq(
        i: usize,
        forward: bool,
        nums: &Vec<i32>,
        c: usize,
        k: usize,
        v: usize,
        app: &mut Vec<Vec<Vec<bool>>>,
        val: &mut Vec<usize>,
    ) {
        if c == 0 {
            if forward {
                val[v] = val[v].min(i - 1);
            } else {
                val[v] = val[v].max(i + 1);
            }
            return;
        }
        if ((forward && i + k + c <= nums.len()) || (!forward && i >= k + c - 1)) && !app[i][c][v] {
            app[i][c][v] = true;
            let ni = if forward { i + 1 } else { i - 1 };
            Self::seq(ni, forward, nums, c, k, v, app, val);
            Self::seq(ni, forward, nums, c - 1, k, v | nums[i] as usize, app, val);
        }
    }
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let k = k as usize;
        let mut orl = vec![vec![vec![false; 128]; k + 1]; len];
        let mut orr = vec![vec![vec![false; 128]; k + 1]; len];
        let mut vall = vec![len; 128];
        let mut valr = vec![0; 128];
        Self::seq(0, true, &nums, k, k, 0, &mut orl, &mut vall);
        Self::seq(len - 1, false, &nums, k, k, 0, &mut orr, &mut valr);
        let mut res = 0;
        for i in 1..128 {
            for j in 1..128 {
                if vall[i] < valr[j] {
                    res = res.max(i ^ j);
                }
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_value() {
        assert_eq!(Solution::max_value(vec![2, 14, 36, 61, 104, 49], 3), 83);
        assert_eq!(Solution::max_value(vec![1, 89, 11, 90], 2), 2);
        assert_eq!(Solution::max_value(vec![2, 6, 7], 1), 5);
        assert_eq!(Solution::max_value(vec![4, 2, 5, 6, 7], 2), 2);
    }
}
