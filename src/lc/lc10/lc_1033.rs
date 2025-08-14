// https://leetcode.com/problems/moving-stones-until-consecutive/
// 1033. Moving Stones Until Consecutivepub struct Solution
pub struct Solution;
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut stones = vec![a, b, c];
        stones.sort();
        let max_move = stones[2] - stones[0] - 2;
        let mut step = vec![stones[1] - stones[0], stones[2] - stones[1]];
        step.sort();
        let min_move = if step[1] == 1 {
            0
        } else if step[0] == 1 || step[0] == 2 {
            1
        } else {
            2
        };
        vec![min_move, max_move]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_moves_stones() {
        assert_eq!(Solution::num_moves_stones(1, 2, 5), vec![1, 2]);
        assert_eq!(Solution::num_moves_stones(4, 3, 2), vec![0, 0]);
        assert_eq!(Solution::num_moves_stones(3, 5, 1), vec![1, 2]);
    }
}
