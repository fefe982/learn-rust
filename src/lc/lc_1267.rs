// https://leetcode.com/problems/count-servers-that-communicate/
// 1267. Count Servers that Communicate
pub struct Solution;
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt_r = vec![0; grid.len()];
        let mut cnt_c = vec![0; grid[0].len()];
        let mut cnt = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    cnt_r[i] += 1;
                    cnt_c[j] += 1;
                    cnt += 1;
                }
            }
        }
        for i in 0..grid.len() {
            if cnt_r[i] != 1 {
                continue;
            }
            for j in 0..grid[i].len() {
                if cnt_c[j] == 1 && grid[i][j] == 1 {
                    cnt -= 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_servers() {
        assert_eq!(
            Solution::count_servers(vec_vec![
                [0, 0, 1, 0, 1],
                [0, 1, 0, 1, 0],
                [0, 1, 1, 1, 0],
                [1, 0, 0, 1, 1],
                [0, 0, 1, 1, 0]
            ]),
            12
        );
        assert_eq!(Solution::count_servers(vec_vec![[1, 0], [0, 1]]), 0);
        assert_eq!(
            Solution::count_servers(vec_vec![[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
            4
        );
    }
}
