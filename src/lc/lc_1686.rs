// https://leetcode.com/problems/stone-game-vi/
// 1686. Stone Game VI
pub struct Solution;
impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut v = alice_values.into_iter().zip(bob_values).collect::<Vec<_>>();
        v.sort_by_key(|x| x.0 + x.1);
        let mut d = 0;
        for (i, (a, b)) in v.into_iter().rev().enumerate() {
            if i % 2 == 0 {
                d += a;
            } else {
                d -= b;
            }
        }
        match d.cmp(&0) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stone_game_vi() {
        assert_eq!(Solution::stone_game_vi(vec![1, 3], vec![2, 1]), 1);
        assert_eq!(Solution::stone_game_vi(vec![1, 2], vec![3, 1]), 0);
        assert_eq!(Solution::stone_game_vi(vec![2, 4, 2], vec![1, 6, 7]), -1)
    }
}
