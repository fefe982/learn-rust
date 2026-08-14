// https://leetcode.com/problems/minimize-length-of-array-using-operations/
// 3012. Minimize Length of Array Using Operations
pub struct Solution;
impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().min().unwrap();
        let mut n = 0;
        for i in nums {
            if i == m {
                n += 1;
            } else if i % m > 0 {
                return 1;
            }
        }
        (n + 1) / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_array_length() {
        assert_eq!(Solution::minimum_array_length(vec![1, 4, 3, 1]), 1);
        assert_eq!(Solution::minimum_array_length(vec![5, 5, 5, 10, 5]), 2);
        assert_eq!(Solution::minimum_array_length(vec![2, 3, 4]), 1);
    }
}
