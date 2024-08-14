// https://leetcode.com/problems/special-array-ii/
// 3152. Special Array II
pub struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut nums = nums;
        let mut s = 0;
        let mut e = 1;
        while e < nums.len() {
            if nums[e] % 2 == nums[e - 1] % 2 {
                for i in s..e {
                    nums[i] = (e - 1) as i32;
                }
                s = e;
            }
            e += 1;
        }
        for i in s..e {
            nums[i] = (e - 1) as i32;
        }
        queries.into_iter().map(|q| nums[q[0] as usize] >= q[1]).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_array_special() {
        assert_eq!(
            Solution::is_array_special(vec![3, 4, 1, 2, 6], vec_vec![[0, 4]]),
            [false]
        );
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], vec_vec![[0, 2], [2, 3]]),
            [false, true]
        );
    }
}
