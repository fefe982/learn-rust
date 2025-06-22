// https://leetcode.com/problems/trapping-rain-water-ii/
// 407. Trapping Rain Water II
pub struct Solution;
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let h = height_map.len();
        let w = height_map[0].len();
        if h <= 2 || w <= 2 {
            return 0;
        }
        let mut visited = vec![vec![false; w]; h];
        let mut q = std::collections::BinaryHeap::new();
        for i in 0..h {
            q.push((std::cmp::Reverse(height_map[i][0]), i, 0));
            q.push((std::cmp::Reverse(height_map[i][w - 1]), i, w - 1));
            visited[i][0] = true;
            visited[i][w - 1] = true;
        }
        for i in 1..w - 1 {
            q.push((std::cmp::Reverse(height_map[0][i]), 0, i));
            q.push((std::cmp::Reverse(height_map[h - 1][i]), h - 1, i));
            visited[0][i] = true;
            visited[h - 1][i] = true;
        }
        let mut nwater = 0;
        let mut level = 0;
        while let Some((std::cmp::Reverse(height), i, j)) = q.pop() {
            nwater += (level - height).max(0);
            level = level.max(height);
            for d in [[0, -1], [-1, 0], [1, 0], [0, 1]] {
                let ni = (i as i32 + d[0]) as usize;
                let nj = (j as i32 + d[1]) as usize;
                if ni < h && nj < w && !visited[ni][nj] {
                    q.push((std::cmp::Reverse(height_map[ni][nj]), ni, nj));
                    visited[ni][nj] = true;
                }
            }
        }
        nwater
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn trap_rain_water() {
        assert_eq!(
            Solution::trap_rain_water(vec_vec![[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]]),
            4
        );
        assert_eq!(
            Solution::trap_rain_water(vec_vec![
                [3, 3, 3, 3, 3],
                [3, 2, 2, 2, 3],
                [3, 2, 1, 2, 3],
                [3, 2, 2, 2, 3],
                [3, 3, 3, 3, 3]
            ]),
            10
        );
    }
}
