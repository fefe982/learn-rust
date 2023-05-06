// https://leetcode.com/problems/dungeon-game/
// 174. Dungeon Game
pub struct Solution;
impl Solution {
    pub fn calculate_minimum_hp(mut dungeon: Vec<Vec<i32>>) -> i32 {
        let r = dungeon.len() - 1;
        let c = dungeon[0].len() - 1;
        dungeon[r][c] = std::cmp::max(1, 1 - dungeon[r][c]);
        for j in (0..c).rev() {
            dungeon[r][j] = std::cmp::max(1, dungeon[r][j + 1] - dungeon[r][j]);
        }
        for i in (0..r).rev() {
            dungeon[i][c] = std::cmp::max(1, dungeon[i + 1][c] - dungeon[i][c]);
            for j in (0..c).rev() {
                dungeon[i][j] = std::cmp::min(
                    std::cmp::max(1, dungeon[i + 1][j] - dungeon[i][j]),
                    std::cmp::max(1, dungeon[i][j + 1] - dungeon[i][j]),
                );
            }
        }
        dungeon[0][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn calculate_minimum_hp() {
        assert_eq!(Solution::calculate_minimum_hp(vec_vec![[-3], [-7]]), 11);
        assert_eq!(
            Solution::calculate_minimum_hp(vec_vec![[-2, -3, 3], [-5, -10, 1], [10, 30, -5]]),
            7
        );
        assert_eq!(Solution::calculate_minimum_hp(vec_vec![[0]]), 1);
    }
}
