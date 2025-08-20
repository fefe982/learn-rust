// https://leetcode.com/problems/stone-game-iv/
// 1510. Stone Game IV
pub struct Solution;
impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut v = vec![false; n as usize + 1];
        for i in 0..=n as usize {
            if !v[i] {
                (1..)
                    .map(|j| i + j * j)
                    .take_while(|&k| k <= n as usize)
                    .for_each(|k| v[k] = true);
            }
        }
        v[n as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_winner_square_game() {
        assert_eq!(Solution::winner_square_game(1), true);
        assert_eq!(Solution::winner_square_game(2), false);
        assert_eq!(Solution::winner_square_game(4), true);
    }
}
