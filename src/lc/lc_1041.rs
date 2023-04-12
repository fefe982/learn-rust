// https://leetcode.com/problems/robot-bounded-in-circle/
// 1041. Robot Bounded In Circle
pub struct Solution;
impl Solution {
    fn mul(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
    }
    fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 + b.0, a.1 + b.1)
    }
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut pos = (0, 0);
        let mut dir = (0, 1);
        for &c in instructions.as_bytes() {
            match c {
                b'G' => pos = Self::add(pos, dir),
                b'L' => dir = Self::mul(dir, (0, 1)),
                b'R' => dir = Self::mul(dir, (0, -1)),
                _ => {}
            }
        }
        pos == (0, 0) || dir != (0, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_robot_bounded() {
        assert_eq!(Solution::is_robot_bounded(String::from("GGLLGG")), true);
        assert_eq!(Solution::is_robot_bounded(String::from("GG")), false);
    }
}
