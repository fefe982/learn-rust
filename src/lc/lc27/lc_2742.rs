// https://leetcode.com/problems/painting-the-walls/
// 2742. Painting the Walls
pub struct Solution;
impl Solution {
    fn paint(
        cost: &Vec<i32>,
        time: &Vec<i32>,
        pos: usize,
        left: i32,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if left <= 0 {
            return 0;
        }
        if pos >= cost.len() {
            return i32::MAX / 2;
        }
        let left = left as usize;
        if memo[pos][left] != -1 {
            return memo[pos][left];
        }
        let res = Self::paint(cost, time, pos + 1, left as i32, memo)
            .min(Self::paint(cost, time, pos + 1, left as i32 - 1 - time[pos], memo) + cost[pos]);
        memo[pos][left] = res;
        res
    }
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        Self::paint(
            &cost,
            &time,
            0,
            cost.len() as i32,
            &mut vec![vec![-1; cost.len() + 1]; cost.len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_paint_walls() {
        assert_eq!(Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
        assert_eq!(Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }
}
