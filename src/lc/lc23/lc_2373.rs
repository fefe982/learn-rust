// https://leetcode.com/problems/largest-local-values-in-a-matrix/
// 2373. Largest Local Values in a Matrix
pub struct Solution;
impl Solution {
    fn push(h: &mut std::collections::BinaryHeap<(i32, usize)>, v: i32, i: usize) -> i32 {
        h.push((v, i));
        while let Some((vtop, itop)) = h.peek() {
            if *itop + 2 < i {
                h.pop();
            } else {
                return *vtop;
            }
        }
        -1
    }
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let mut h = std::collections::BinaryHeap::new();
        for i in 0..grid.len() {
            h.clear();
            h.push((grid[i][0], 0));
            h.push((grid[i][1], 1));
            for j in 0..grid[i].len() - 2 {
                grid[i][j] = Self::push(&mut h, grid[i][j + 2], j + 2);
            }
            let l = grid[i].len();
            grid[i].truncate(l - 2);
        }
        for i in 0..grid[0].len() {
            h.clear();
            h.push((grid[0][i], 0));
            h.push((grid[1][i], 1));
            for j in 0..grid.len() - 2 {
                grid[j][i] = Self::push(&mut h, grid[j + 2][i], j + 2);
            }
        }
        grid.truncate(grid.len() - 2);
        grid
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_largest_local() {
        assert_eq!(
            Solution::largest_local(vec_vec![[9, 9, 8, 1], [5, 6, 2, 6], [8, 2, 6, 4], [6, 2, 2, 2]]),
            vec_vec![[9, 9], [8, 6]]
        );
        assert_eq!(
            Solution::largest_local(vec_vec![
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1],
                [1, 1, 2, 1, 1],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1]
            ]),
            vec_vec![[2, 2, 2], [2, 2, 2], [2, 2, 2]]
        );
    }
}
