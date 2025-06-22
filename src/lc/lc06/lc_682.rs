// https://leetcode.com/problems/baseball-game/
// 682. Baseball Game
pub struct Solution;
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for op in operations {
            match op.as_str() {
                "C" => {
                    stack.pop();
                }
                "D" => {
                    stack.push(stack.last().unwrap().clone() * 2);
                }
                "+" => {
                    stack.push(stack.last().unwrap().clone() + stack[stack.len() - 2].clone());
                }
                _ => {
                    stack.push(op.parse::<i32>().unwrap());
                }
            }
        }
        stack.iter().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cal_points() {
        assert_eq!(Solution::cal_points(vec_str!["5", "2", "C", "D", "+"]), 30);
        assert_eq!(
            Solution::cal_points(vec_str!["5", "-2", "4", "C", "D", "9", "+", "+"]),
            27
        );
        assert_eq!(Solution::cal_points(vec_str!["1", "C"]), 0);
    }
}
