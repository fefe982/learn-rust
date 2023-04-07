// https://leetcode.com/problems/first-missing-positive/
// 41. First Missing Positive
pub struct Solution;
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for idx in 0..nums.len() {
            let n = nums[idx];
            if n < 0 || n > nums.len() as i32 {
                nums[idx] = 0
            }
        }
        let base = nums.len() as i32 + 1;
        for idx in 0..nums.len() {
            let n = nums[idx] % base;
            if n != 0 {
                if nums[n as usize - 1] < base {
                    nums[n as usize - 1] += base;
                }
            }
        }
        for (idx, &n) in nums.iter().enumerate() {
            if n < base {
                return idx as i32 + 1;
            }
        }
        return nums.len() as i32 + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_missing_positive() {
        assert_eq!(
            Solution::first_missing_positive(vec![
                11, 1, 6, 11, 5, 5, -6, 9, -3, 9, 5, 4, 2, -8, 16, -6, -4, 2, 3
            ]),
            7
        );
        assert_eq!(Solution::first_missing_positive(vec![4, 1, 3, 3]), 2);
        assert_eq!(Solution::first_missing_positive(vec![1, 3, 3]), 2);
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 6, 3, 5, 4]), 7);
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
