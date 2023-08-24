// https://leetcode.com/problems/count-servers-that-communicate/
// 1267. Count Servers that Communicate
pub struct Solution;
impl Solution {
    pub fn count_servers(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != 1 {
                    continue;
                }
                let mut q = vec![(i, j)];
                let mut c = 0;
                while let Some((ii, jj)) = q.pop() {
                    if grid[ii][jj] != 1 {
                        continue;
                    }
                    grid[ii][jj] = -1;
                    c += 1;
                    for iii in 0..grid.len() {
                        if grid[iii][jj] == 1 {
                            q.push((iii, jj));
                        }
                    }
                    for jjj in 0..grid[ii].len() {
                        if grid[ii][jjj] == 1 {
                            q.push((ii, jjj));
                        }
                    }
                }
                if c > 1 {
                    cnt += c;
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
            Solution::count_servers(vec_vec![
                [1, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1]
            ]),
            4
        );
    }
}
