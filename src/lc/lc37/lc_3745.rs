// https://leetcode.com/problems/maximize-expression-of-three-elements/
// 3745. Maximize Expression of Three Elements
pub struct Solution;
impl Solution {
    pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        for n in nums {
            min = min.min(n);
            if n > max2 {
                if n > max1 {
                    max2 = max1;
                    max1 = n;
                } else {
                    max2 = n;
                }
            }
        }
        max1 + max2 - min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_expression_of_three() {
        assert_eq!(Solution::maximize_expression_of_three(vec![1, 4, 2, 5]), 8);
        assert_eq!(Solution::maximize_expression_of_three(vec![-2, 0, 5, -2, 4]), 11);
    }
}
