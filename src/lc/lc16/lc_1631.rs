// https://leetcode.com/problems/path-with-minimum-effort/
// 1631. Path With Minimum Effort
pub struct Solution;
impl Solution {
    pub fn minimum_effort_path(mut heights: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), 0, 0));
        while let Some((m, x, y)) = q.pop() {
            let h = heights[x][y];
            if h < 0 {
                continue;
            }
            heights[x][y] = -h;
            max = max.max(m.0);
            if x == heights.len() - 1 && y == heights[0].len() - 1 {
                return max;
            }
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = (x as i32 + dir.0) as usize;
                let ny = (y as i32 + dir.1) as usize;
                if nx < heights.len() && ny < heights[0].len() && heights[nx][ny] > 0 {
                    q.push((std::cmp::Reverse((h - heights[nx][ny]).abs()), nx, ny));
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_effort_path() {
        assert_eq!(
            Solution::minimum_effort_path(vec_vec![[1, 2, 2], [3, 8, 2], [5, 3, 5]]),
            2
        );
        assert_eq!(
            Solution::minimum_effort_path(vec_vec![[1, 2, 3], [3, 8, 4], [5, 3, 5]]),
            1
        );
        assert_eq!(
            Solution::minimum_effort_path(vec_vec![
                [1, 2, 1, 1, 1],
                [1, 2, 1, 2, 1],
                [1, 2, 1, 2, 1],
                [1, 2, 1, 2, 1],
                [1, 1, 1, 2, 1]
            ]),
            0
        );
    }
}
