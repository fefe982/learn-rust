// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/
// 2610. Convert an Array Into a 2D Array With Conditions
pub struct Solution;
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = Vec::new();
        let mut cur = -1;
        let mut row = 0;
        for n in nums {
            if n != cur {
                row = 0;
            } else {
                row += 1;
            }
            if row >= ans.len() {
                ans.push(vec![n]);
            } else {
                ans[row].push(n);
            }
            cur = n;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_matrix() {
        assert_eq!(
            Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
            vec_vec![[1, 2, 3, 4], [1, 3], [1]]
        );
        assert_eq!(Solution::find_matrix(vec![1, 2, 3, 4]), vec_vec![[1, 2, 3, 4]]);
    }
}
