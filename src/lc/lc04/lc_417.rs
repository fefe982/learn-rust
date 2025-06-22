// https://leetcode.com/problems/pacific-atlantic-water-flow/
// 417. Pacific Atlantic Water Flow
pub struct Solution;
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut fill = vec![vec![0; heights[0].len()]; heights.len()];
        let mut res = vec![];
        let mut q = vec![];
        for i in 0..heights.len() {
            q.push((i, 0));
            fill[i][0] = 5;
            if fill[i][heights[0].len() - 1] == 0 {
                q.push((i, heights[0].len() - 1));
            }
            fill[i][heights[0].len() - 1] |= 6;
        }
        for i in 0..heights[0].len() {
            if fill[0][i] == 0 {
                q.push((0, i));
            }
            fill[0][i] |= 5;
            if fill[heights.len() - 1][i] == 0 {
                q.push((heights.len() - 1, i));
            }
            fill[heights.len() - 1][i] |= 6;
        }
        fill[0][heights[0].len() - 1] = 7;
        fill[heights.len() - 1][0] = 7;
        while let Some((x, y)) = q.pop() {
            fill[x][y] &= 3;
            if fill[x][y] == 3 {
                res.push(vec![x as i32, y as i32]);
            }
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx >= heights.len()
                    || ny >= heights[0].len()
                    || heights[nx][ny] < heights[x][y]
                    || fill[nx][ny] & fill[x][y] == fill[x][y]
                {
                    continue;
                }
                fill[nx][ny] |= fill[x][y];
                if fill[nx][ny] & 4 == 0 {
                    fill[nx][ny] |= 4;
                    q.push((nx, ny));
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn pacific_atlantic() {
        assert_sort_eq!(
            Solution::pacific_atlantic(vec_vec![
                [13],
                [4],
                [19],
                [10],
                [1],
                [11],
                [5],
                [17],
                [3],
                [10],
                [1],
                [0],
                [1],
                [4],
                [1],
                [3],
                [6],
                [13],
                [2],
                [16],
                [7],
                [6],
                [3],
                [1],
                [9],
                [9],
                [13],
                [10],
                [9],
                [10],
                [6],
                [2],
                [11],
                [17],
                [13],
                [0],
                [19],
                [7],
                [13],
                [3],
                [9],
                [2]
            ]),
            vec_vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [6, 0],
                [7, 0],
                [8, 0],
                [9, 0],
                [10, 0],
                [11, 0],
                [12, 0],
                [13, 0],
                [14, 0],
                [15, 0],
                [16, 0],
                [17, 0],
                [18, 0],
                [19, 0],
                [20, 0],
                [21, 0],
                [22, 0],
                [23, 0],
                [24, 0],
                [25, 0],
                [26, 0],
                [27, 0],
                [28, 0],
                [29, 0],
                [30, 0],
                [31, 0],
                [32, 0],
                [33, 0],
                [34, 0],
                [35, 0],
                [36, 0],
                [37, 0],
                [38, 0],
                [39, 0],
                [40, 0],
                [41, 0]
            ]
        );
        assert_sort_eq!(
            Solution::pacific_atlantic(vec_vec![
                [1, 2, 2, 3, 5],
                [3, 2, 3, 4, 4],
                [2, 4, 5, 3, 2],
                [6, 7, 1, 4, 5],
                [5, 1, 1, 2, 4]
            ]),
            vec_vec![[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        );
        assert_sort_eq!(Solution::pacific_atlantic(vec_vec![[1]]), vec_vec![[0, 0]]);
    }
}
