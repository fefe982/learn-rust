// https://leetcode.cn/problems/zero-array-transformation-i/
// 3355. Zero Array Transformation I
pub struct Solution;
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        let mut j = 0;
        let mut dec = 0;
        for i in 0..nums.len() {
            while nums[i] - dec > 0 && j < queries.len() {
                if queries[j][1] as usize >= i {
                    if queries[j][0] as usize <= i {
                        dec += 1;
                    } else {
                        v[queries[j][0] as usize] += 1;
                    }
                    v[queries[j][1] as usize + 1] -= 1;
                }
                j += 1;
            }
            if nums[i] - dec > 0 {
                return false;
            }
            dec += v[i + 1];
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_zero_array() {
        assert_eq!(Solution::is_zero_array(vec![1, 0, 1], vec_vec![[0, 2]]), true);
        assert_eq!(
            Solution::is_zero_array(vec![4, 3, 2, 1], vec_vec![[1, 3], [0, 2]]),
            false
        )
    }
}
