// https://leetcode.com/problems/find-missing-elements/
// 3731. Find Missing Elements
pub struct Solution;
impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = vec![];
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            for j in 1..diff {
                res.push(nums[i - 1] + j);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_missing_elements() {
        assert_eq!(Solution::find_missing_elements(vec![1, 4, 2, 5]), vec![3]);
        assert_eq!(Solution::find_missing_elements(vec![7, 8, 6, 9]), vec![]);
        assert_eq!(Solution::find_missing_elements(vec![5, 1]), vec![2, 3, 4]);
    }
}
