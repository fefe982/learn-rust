// https://leetcode.com/problems/calculate-score-after-performing-instructions/
// 3522. Calculate Score After Performing Instructions
pub struct Solution;
impl Solution {
    pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut i = 0;
        let mut instructions = instructions;
        while i < instructions.len() {
            match instructions[i].as_str() {
                "jump" => {
                    instructions[i] = "".to_string();
                    i = (i as i32 + values[i]) as usize;
                }
                "add" => {
                    instructions[i] = "".to_string();
                    res += values[i] as i64;
                    i += 1;
                }
                _ => break,
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn calculate_score() {
        assert_eq!(
            Solution::calculate_score(
                vec_str!["jump", "add", "add", "jump", "add", "jump"],
                vec![2, 1, 3, 1, -2, -3]
            ),
            1
        );
        assert_eq!(
            Solution::calculate_score(vec_str!["jump", "add", "add"], vec![3, 1, 1]),
            0
        );
        assert_eq!(Solution::calculate_score(vec_str!["jump"], vec![0]), 0);
    }
}
