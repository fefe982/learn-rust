// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
// 2011. Final Value of Variable After Performing Operations
pub struct Solution;
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().fold(0, |acc, x| match x.as_str() {
            "--X" => acc - 1,
            "X--" => acc - 1,
            "++X" => acc + 1,
            "X++" => acc + 1,
            _ => acc,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn final_value_after_operations() {
        assert_eq!(Solution::final_value_after_operations(vec_str!["--X", "X++", "X++"]), 1);
        assert_eq!(Solution::final_value_after_operations(vec_str!["++X", "++X", "X++"]), 3);
        assert_eq!(
            Solution::final_value_after_operations(vec_str!["X++", "++X", "--X", "X--"]),
            0
        );
    }
}
