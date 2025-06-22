// https://leetcode.com/problems/robot-return-to-origin/
// 657. Robot Return to Origin
pub struct Solution;
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut u = 0;
        let mut l = 0;
        for c in moves.chars() {
            match c {
                'U' => u += 1,
                'D' => u -= 1,
                'L' => l += 1,
                'R' => l -= 1,
                _ => (),
            }
        }
        u == 0 && l == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn judge_circle() {
        assert_eq!(Solution::judge_circle("UD".to_string()), true);
        assert_eq!(Solution::judge_circle("LL".to_string()), false);
    }
}
