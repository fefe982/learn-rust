// https://leetcode.com/problems/sum-of-beauty-in-the-array/
// 2012. Sum of Beauty in the Array
pub struct Solution;
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut v = vec![];
        let mut res = 0;
        for i in 1..nums.len() {
            while let Some(&n) = v.last() {
                if n >= nums[i] {
                    v.pop();
                } else {
                    break;
                }
            }
            if i + 1 < nums.len() && nums[i] > max {
                v.push(nums[i]);
            }
            max = max.max(nums[i]);
            if i + 1 < nums.len() && nums[i] > nums[i - 1] && nums[i] < nums[i + 1] {
                res += 1;
            }
        }
        res + v.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_beauties() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 2, 3]), 2);
        assert_eq!(Solution::sum_of_beauties(vec![2, 4, 6, 4]), 1);
        assert_eq!(Solution::sum_of_beauties(vec![3, 2, 1]), 0);
    }
}
