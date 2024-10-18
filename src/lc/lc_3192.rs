// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-ii/
// 3192. Minimum Operations to Make Binary Array Elements Equal to One II
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(0, |acc, x| if acc % 2 == x % 2 { acc + 1 } else { acc })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 0, 1]), 4);
        assert_eq!(Solution::min_operations(vec![1, 0, 0, 0]), 1);
    }
}
