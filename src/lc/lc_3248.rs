// https://leetcode.com/problems/snake-in-matrix/
// 3248. Snake in a Matrix
pub struct Solution;
impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        for c in commands {
            match c.as_str() {
                "RIGHT" => j += 1,
                "LEFT" => j -= 1,
                "UP" => i -= 1,
                "DOWN" => i += 1,
                _ => (),
            }
        }
        i * n + j
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_final_position_of_snake() {
        assert_eq!(Solution::final_position_of_snake(2, vec_str!["RIGHT", "DOWN"]), 3);
        assert_eq!(Solution::final_position_of_snake(3, vec_str!["DOWN", "RIGHT", "UP"]), 1);
    }
}
