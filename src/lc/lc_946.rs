// https://leetcode.com/problems/validate-stack-sequences/
// 946. Validate Stack Sequences
pub struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut pop_index = 0;
        for val in pushed {
            stack.push(val);
            while let Some(&top) = stack.last() {
                if top == popped[pop_index] {
                    stack.pop();
                    pop_index += 1;
                } else {
                    break;
                }
            }
        }
        stack.len() == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_stack_sequences() {
        assert_eq!(
            Solution::validate_stack_sequences(Vec::from_iter(1..6), vec![4, 5, 3, 2, 1]),
            true
        );
        assert_eq!(
            Solution::validate_stack_sequences(Vec::from_iter(1..6), vec![4, 3, 5, 1, 2]),
            false
        );
    }
}
