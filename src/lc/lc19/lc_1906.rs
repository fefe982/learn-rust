// https://leetcode.com/problems/minimum-absolute-difference-queries/
// 1906. Minimum Absolute Difference Queries
pub struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut pre = vec![[0_i32; 101]; n + 1];
        for i in 0..n {
            pre[i + 1] = pre[i];
            pre[i + 1][nums[i] as usize] += 1;
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize + 1;

            let mut prev = -1;
            let mut best = i32::MAX;
            for v in 1..=100 {
                if pre[r][v] - pre[l][v] > 0 {
                    if prev != -1 {
                        best = best.min(v as i32 - prev);
                    }
                    prev = v as i32;
                }
            }
            if best == i32::MAX {
                ans.push(-1);
            } else {
                ans.push(best);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_difference() {
        assert_eq!(
            Solution::min_difference(vec![1, 3, 4, 8], vec_vec![[0, 1], [1, 2], [2, 3], [0, 3]]),
            vec![2, 1, 4, 1]
        );
        assert_eq!(
            Solution::min_difference(vec![4, 5, 2, 2, 7, 10], vec_vec![[2, 3], [0, 2], [0, 5], [3, 5]]),
            vec![-1, 1, 1, 3]
        );
    }
}
