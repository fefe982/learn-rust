// https://leetcode.com/problems/minimum-moves-to-reach-target-score/
// 2139. Minimum Moves to Reach Target Score
pub struct Solution;
impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut target = target;
        let mut max_doubles = max_doubles;
        let mut moves = 0;
        while target > 1 && max_doubles > 0 {
            if target % 2 == 0 {
                target /= 2;
                max_doubles -= 1;
            } else {
                target -= 1;
            }
            moves += 1;
        }
        moves + target - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_moves() {
        assert_eq!(Solution::min_moves(5, 0), 4);
        assert_eq!(Solution::min_moves(19, 2), 7);
        assert_eq!(Solution::min_moves(10, 4), 4);
    }
}
