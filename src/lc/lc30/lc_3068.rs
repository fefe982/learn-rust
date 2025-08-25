// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/
// 3068. Find the Maximum Sum of Node Values
pub struct Solution;
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut npos = 0;
        let mut nneg = 0;
        let mut minpos = i64::MAX;
        let mut maxneg = i64::MIN;
        let mut sum = 0;
        let mut sum_inc = 0;
        for n in nums {
            let nx = ((n ^ k) - n) as i64;
            sum += n as i64;
            if nx > 0 {
                minpos = minpos.min(nx);
                sum_inc += nx;
                npos += 1;
            } else {
                maxneg = maxneg.max(nx);
                nneg += 1;
            }
        }
        if npos % 2 == 0 {
            sum + sum_inc
        } else if nneg == 0 || minpos + maxneg <= 0 {
            sum + sum_inc - minpos
        } else {
            sum + sum_inc + maxneg
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_value_sum() {
        assert_eq!(
            Solution::maximum_value_sum(vec![1, 2, 1], 3, vec_vec![[0, 1], [0, 2]]),
            6
        );
        assert_eq!(Solution::maximum_value_sum(vec![2, 3], 7, vec_vec![[0, 1]]), 9);
        assert_eq!(
            Solution::maximum_value_sum(
                vec![7, 7, 7, 7, 7, 7],
                3,
                vec_vec![[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]]
            ),
            42
        );
    }
}
