// https://leetcode.com/problems/moving-stones-until-consecutive-ii/
// 1040. Moving Stones Until Consecutive II
pub struct Solution;
impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort();
        let l = stones.len();
        let max_move =
            std::cmp::max(stones[l - 2] - stones[0], stones[l - 1] - stones[1]) - l as i32 + 2;
        let mut low = 0;
        let mut high = 1;
        let mut min_move = i32::MAX;
        loop {
            while high < l && stones[high] - stones[low] < l as i32 - 1 {
                high += 1;
            }
            if high >= l {
                break;
            }
            if stones[high] - stones[low] == l as i32 - 1 {
                min_move = std::cmp::min(min_move, l as i32 - 1 - (high - low) as i32);
            } else if low != 0 {
                min_move = std::cmp::min(min_move, l as i32 - (high - low) as i32);
            }
            low += 1;
        }
        vec![min_move, max_move]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_moves_stones_ii() {
        assert_eq!(Solution::num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
        assert_eq!(
            Solution::num_moves_stones_ii(vec![6, 5, 4, 3, 10]),
            vec![2, 3]
        );
    }
}
