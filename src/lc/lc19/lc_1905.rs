// https://leetcode.com/problems/count-sub-islands/
// 1905. Count Sub Islands
pub struct Solution;
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut grid2 = grid2;
        let mut cnt = 0;
        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                if grid2[i][j] == 1 {
                    let mut q = vec![(i, j)];
                    grid2[i][j] = 0;
                    let mut sub = true;
                    while let Some((ci, cj)) = q.pop() {
                        if grid1[ci][cj] == 0 {
                            sub = false;
                        }
                        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let (ni, nj) = ((ci as i32 + di) as usize, (cj as i32 + dj) as usize);
                            if ni < grid2.len() && nj < grid2[0].len() && grid2[ni][nj] == 1 {
                                q.push((ni, nj));
                                grid2[ni][nj] = 0;
                            }
                        }
                    }
                    if sub {
                        cnt += 1;
                    }
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
    fn test_count_sub_islands() {
        assert_eq!(
            Solution::count_sub_islands(
                vec_vec![
                    [1, 1, 1, 0, 0],
                    [0, 1, 1, 1, 1],
                    [0, 0, 0, 0, 0],
                    [1, 0, 0, 0, 0],
                    [1, 1, 0, 1, 1]
                ],
                vec_vec![
                    [1, 1, 1, 0, 0],
                    [0, 0, 1, 1, 1],
                    [0, 1, 0, 0, 0],
                    [1, 0, 1, 1, 0],
                    [0, 1, 0, 1, 0]
                ],
            ),
            3
        );
        assert_eq!(
            Solution::count_sub_islands(
                vec_vec![
                    [1, 0, 1, 0, 1],
                    [1, 1, 1, 1, 1],
                    [0, 0, 0, 0, 0],
                    [1, 1, 1, 1, 1],
                    [1, 0, 1, 0, 1]
                ],
                vec_vec![
                    [0, 0, 0, 0, 0],
                    [1, 1, 1, 1, 1],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [1, 0, 0, 0, 1]
                ],
            ),
            2
        );
    }
}
