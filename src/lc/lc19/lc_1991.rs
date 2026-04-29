// https://leetcode.com/problems/find-the-middle-index-in-array/
// 1991. Find the Middle Index in Array
pub struct Solution;
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            if left_sum * 2 + num == sum {
                return i as i32;
            }
            left_sum += num;
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_middle_index() {
        assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
        assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
        assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
    }
}
