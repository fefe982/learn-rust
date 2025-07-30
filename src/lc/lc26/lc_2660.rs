// https://leetcode.com/problems/determine-the-winner-of-a-bowling-game/
// 2660. Determine the Winner of a Bowling Game
pub struct Solution;
use std::cmp::Ordering;
impl Solution {
    fn score(p: &Vec<i32>) -> i32 {
        let mut last = -2;
        let mut sum = 0;
        for &s in p {
            if last < -1 {
                sum += s;
            } else {
                sum += s * 2;
            }
            if s == 10 {
                last = 0;
            } else {
                last -= 1;
            }
        }
        sum
    }
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        match Solution::score(&player1).cmp(&Solution::score(&player2)) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => 2,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_winner() {
        assert_eq!(Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]), 1);
        assert_eq!(Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2);
        assert_eq!(Solution::is_winner(vec![2, 3], vec![4, 1]), 0);
    }
}
