// https://leetcode.com/problems/elimination-game/
// 390. Elimination Game
pub struct Solution;
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut c = n;
        let mut f = 1;
        let mut step = 1;
        while c > 1 {
            f += step;
            c /= 2;
            step *= 2;
            if c == 1 {
                break;
            }
            if c % 2 == 1 {
                f += step;
            }
            c /= 2;
            step *= 2;
        }
        f
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_remaining() {
        assert_eq!(Solution::last_remaining(9), 6);
        assert_eq!(Solution::last_remaining(1), 1);
    }
}
