// https://leetcode.com/problems/paint-house-iii/
// 1473. Paint House III
pub struct Solution;
impl Solution {
    fn cost(
        houses: &Vec<i32>,
        cost: &Vec<Vec<i32>>,
        target: i32,
        idx: usize,
        color: usize,
        memo: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if idx == houses.len() && target == 0 {
            return 0;
        }
        if target < 0 || (idx + target as usize) > houses.len() {
            return i32::MAX;
        }
        if memo[idx][target as usize][color] >= 0 {
            return memo[idx][target as usize][color];
        }
        let ans = if houses[idx] == 0 {
            let mut min = i32::MAX;
            for c in 1..=cost[0].len() {
                let mut t = Self::cost(houses, cost, target - if c == color { 0 } else { 1 }, idx + 1, c, memo);
                if t != i32::MAX {
                    t += cost[idx][c - 1];
                }
                min = min.min(t);
            }
            min
        } else {
            Self::cost(
                houses,
                cost,
                target - if houses[idx] as usize == color { 0 } else { 1 },
                idx + 1,
                houses[idx] as usize,
                memo,
            )
        };
        memo[idx][target as usize][color] = ans;
        ans
    }
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, _m: i32, _n: i32, target: i32) -> i32 {
        let t = Self::cost(
            &houses,
            &cost,
            target,
            0,
            0,
            &mut vec![vec![vec![-1; cost[0].len() + 1]; target as usize + 1]; houses.len()],
        );
        if t == i32::MAX {
            -1
        } else {
            t
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 0, 0, 0, 0],
                vec_vec![[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]],
                5,
                2,
                3
            ),
            9
        );
        assert_eq!(
            Solution::min_cost(
                vec![0, 2, 1, 2, 0],
                vec_vec![[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]],
                5,
                2,
                3
            ),
            11
        );
        assert_eq!(
            Solution::min_cost(
                vec![3, 1, 2, 3],
                vec_vec![[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
                4,
                3,
                3
            ),
            -1
        );
    }
}
