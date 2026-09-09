// https://leetcode.com/problems/stone-removal-game/
// 3360. Stone Removal Game
pub struct Solution;
impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut k = 10;
        let mut n = n;
        loop {
            if k > n {
                return false;
            }
            n -= k;
            k -= 1;
            if k > n {
                return true;
            }
            n -= k;
            k -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_alice_win() {
        assert_eq!(Solution::can_alice_win(12), true);
        assert_eq!(Solution::can_alice_win(1), false);
    }
}
