// https://leetcode.com/problems/rearrange-array-elements-by-sign/
// 2149. Rearrange Array Elements by Sign
pub struct Solution;
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut r = vec![0; nums.len()];
        let mut i = 0;
        let mut j = 1;
        for num in nums {
            if num > 0 {
                r[i] = num;
                i += 2;
            } else {
                r[j] = num;
                j += 2;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rearrange_array() {
        assert_eq!(
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
        assert_eq!(Solution::rearrange_array(vec![-1, 1]), vec![1, -1]);
    }
}
