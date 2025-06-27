// https://leetcode.com/problems/bricks-falling-when-hit/
// 803. Bricks Falling When Hit
pub struct Solution;
impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mat = grid.clone();
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![0; hits.len()];
        let mut bricks = vec![vec![0; n]; m];
        for hit in &hits {
            mat[hit[0] as usize][hit[1] as usize] = 0
        }
        for j in 0..n {
            if mat[0][j] == 0 || bricks[0][j] == 1 {
                continue;
            }
            bricks[0][j] = 1;
            let mut q = vec![(0, j)];
            while let Some((i, j)) = q.pop() {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = (i as i32 + dx) as usize;
                    let ny = (j as i32 + dy) as usize;
                    if nx < m && ny < n && mat[nx][ny] == 1 && bricks[nx][ny] == 0 {
                        bricks[nx][ny] = 1;
                        q.push((nx, ny));
                    }
                }
            }
        }
        for (i, hit) in hits.into_iter().enumerate().rev() {
            if grid[hit[0] as usize][hit[1] as usize] == 0 {
                continue;
            }
            mat[hit[0] as usize][hit[1] as usize] = 1;
            let mut connected = hit[0] == 0;
            if !connected {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = (hit[0] + dx) as usize;
                    let ny = (hit[1] + dy) as usize;
                    if nx < m && ny < n && bricks[nx][ny] == 1 {
                        connected = true;
                        break;
                    }
                }
            }
            if !connected {
                continue;
            }
            bricks[hit[0] as usize][hit[1] as usize] = 1;
            let mut cnt = 0;
            let mut q = vec![(hit[0] as usize, hit[1] as usize)];
            while let Some((i, j)) = q.pop() {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = (i as i32 + dx) as usize;
                    let ny = (j as i32 + dy) as usize;
                    if nx < m && ny < n && mat[nx][ny] == 1 && bricks[nx][ny] == 0 {
                        bricks[nx][ny] = 1;
                        q.push((nx, ny));
                        cnt += 1;
                    }
                }
            }
            ans[i] = cnt;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_hit_bricks() {
        assert_eq!(
            Solution::hit_bricks(
                vec_vec![[1], [1], [1], [1], [1]],
                vec_vec![[3, 0], [4, 0], [1, 0], [2, 0], [0, 0]]
            ),
            vec![1, 0, 1, 0, 0]
        );
        assert_eq!(
            Solution::hit_bricks(vec_vec![[1, 0, 0, 0], [1, 1, 1, 0]], vec_vec![[1, 0]]),
            vec![2]
        );
        assert_eq!(
            Solution::hit_bricks(vec_vec![[1, 0, 0, 0], [1, 1, 0, 0]], vec_vec![[1, 1], [1, 0]]),
            vec![0, 0]
        );
    }
}
