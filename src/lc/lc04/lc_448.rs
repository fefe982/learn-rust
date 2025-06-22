// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
// 448. Find All Numbers Disappeared in an Array
pub struct Solution;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            while nums[i] != nums[nums[i] as usize - 1] {
                let j = nums[i] as usize - 1;
                nums.swap(i, j);
            }
        }
        let mut res = vec![];
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                res.push(i as i32 + 1);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_disappeared_numbers() {
        assert_eq!(Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]), [5, 6]);
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), [2]);
    }
}
