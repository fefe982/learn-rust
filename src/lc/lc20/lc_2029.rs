// https://leetcode.com/problems/stone-game-ix/
// 2029. Stone Game IX
pub struct Solution;
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut count = [0i32; 3];
        for s in stones {
            count[(s % 3) as usize] += 1;
        }
        if count[1] == 0 || count[2] == 0 {
            count[1] + count[2] > 2 && count[0] % 2 == 1
        } else {
            count[1].abs_diff(count[2]) > 2 || count[0] % 2 == 0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stone_game_ix() {
        assert_eq!(Solution::stone_game_ix(vec![2, 1]), true);
        assert_eq!(Solution::stone_game_ix(vec![2]), false);
        assert_eq!(Solution::stone_game_ix(vec![5, 1, 2, 4, 3]), false);
    }
}
