// https://leetcode.com/problems/merge-operations-for-minimum-travel-time/
// 3538. Merge Operations for Minimum Travel Time
pub struct Solution;
impl Solution {
    fn dfs(
        acc: &Vec<i32>,
        position: &Vec<i32>,
        start: usize,
        end: usize,
        k: usize,
        memo: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if memo[start][end][k] != -1 {
            return memo[start][end][k];
        }
        if end == position.len() {
            if k == 0 {
                return 0;
            } else {
                return i32::MAX / 2;
            }
        }
        let t = acc[end] - acc[start];
        let mut min_time = i32::MAX;
        for i in 0..=k.min(position.len() - end - 1) {
            min_time = min_time.min(
                t * (position[end + i] - position[end - 1]) + Self::dfs(acc, position, end, end + i + 1, k - i, memo),
            );
        }
        memo[start][end][k] = min_time;
        min_time
    }
    pub fn min_travel_time(_l: i32, _n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
        let mut acc = Vec::with_capacity(time.len() + 1);
        acc.push(0);
        for i in 0..time.len() {
            acc.push(acc[i] + time[i]);
        }
        let k = k as usize;
        let mut memo = vec![vec![vec![-1; k + 1]; position.len() + 1]; position.len() + 1];
        Self::dfs(&acc, &position, 0, 1, k, &mut memo)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_travel_time() {
        assert_eq!(
            Solution::min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6]),
            62
        );
        assert_eq!(
            Solution::min_travel_time(5, 5, 1, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]),
            34
        );
    }
}
