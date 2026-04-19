// https://leetcode.com/problems/cyclically-rotating-a-grid/
// 1914. Cyclically Rotating a Grid
pub struct Solution;
impl Solution {
    fn layer_len(layer: usize, m: usize, n: usize) -> usize {
        let h = m - 2 * layer;
        let w = n - 2 * layer;
        (h + w - 2) * 2
    }
    fn index_to_coord(index: usize, layer: usize, m: usize, n: usize) -> (usize, usize) {
        let top = layer;
        let bottom = m - 1 - layer;
        let left = layer;
        let right = n - 1 - layer;
        let width = right - left;
        let height = bottom - top;

        if index <= width {
            (top, left + index)
        } else if index <= width + height {
            (top + (index - width), right)
        } else if index <= 2 * width + height {
            (bottom, right - (index - width - height))
        } else {
            (bottom - (index - 2 * width - height), left)
        }
    }
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = grid.clone();
        for layer in 0..m.min(n) / 2 {
            let len = Self::layer_len(layer, m, n);
            let shift = k as usize % len;
            for i in 0..len {
                let (row, col) = Self::index_to_coord(i, layer, m, n);
                let src = (i + shift) % len;
                let (src_row, src_col) = Self::index_to_coord(src, layer, m, n);
                ans[row][col] = grid[src_row][src_col];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn rotate_grid() {
        assert_eq!(
            Solution::rotate_grid(vec_vec![[40, 10], [30, 20]], 1),
            vec_vec![[10, 20], [40, 30]]
        );
        assert_eq!(
            Solution::rotate_grid(
                vec_vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]],
                2
            ),
            vec_vec![[3, 4, 8, 12], [2, 11, 10, 16], [1, 7, 6, 15], [5, 9, 13, 14]]
        );
        assert_eq!(
            Solution::rotate_grid(vec_vec![[1, 2, 3, 4], [5, 6, 7, 8]], 3),
            vec_vec![[4, 8, 7, 6], [3, 2, 1, 5]]
        );
    }
}
