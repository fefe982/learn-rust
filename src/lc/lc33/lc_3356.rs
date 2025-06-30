// https://leetcode.com/problems/zero-array-transformation-ii/
// 3356. Zero Array Transformation II
pub struct Solution;
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![0; nums.len()];
        let nlen = nums.len();
        let qlen = queries.len();
        let mut q = 0;
        let mut m = 0;
        for i in 0..nlen {
            m += v[i];
            while nums[i] > m {
                if q >= qlen {
                    return -1;
                }
                if (queries[q][1] as usize) < i {
                    q += 1;
                    continue;
                }
                v[queries[q][0] as usize] += queries[q][2];
                if queries[q][1] as usize + 1 < nlen {
                    v[queries[q][1] as usize + 1] -= queries[q][2];
                }
                if queries[q][0] as usize <= i {
                    m += queries[q][2];
                }
                q += 1;
            }
        }
        q as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_zero_array() {
        assert_eq!(
            Solution::min_zero_array(vec![2, 0, 2], vec_vec![[0, 2, 1], [0, 2, 1], [1, 1, 3]]),
            2
        );
        assert_eq!(
            Solution::min_zero_array(vec![4, 3, 2, 1], vec_vec![[1, 3, 2], [0, 2, 1]]),
            -1
        );
    }
}
