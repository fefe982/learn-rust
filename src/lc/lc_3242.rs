// https://leetcode.com/problems/design-neighbor-sum-service/
// 3242. Design Neighbor Sum Service
struct NeighborSum {
    sum: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut sum = vec![(0, 0); grid.len() * grid[0].len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut nsum = 0;
                let mut dsum = 0;
                if i > 0 && j > 0 {
                    dsum += grid[i - 1][j - 1];
                }
                if i > 0 && j + 1 < grid[0].len() {
                    dsum += grid[i - 1][j + 1];
                }
                if i + 1 < grid.len() && j > 0 {
                    dsum += grid[i + 1][j - 1];
                }
                if i + 1 < grid.len() && j + 1 < grid[0].len() {
                    dsum += grid[i + 1][j + 1];
                }
                if j > 0 {
                    nsum += grid[i][j - 1];
                }
                if i > 0 {
                    nsum += grid[i - 1][j];
                }
                if i + 1 < grid.len() {
                    nsum += grid[i + 1][j];
                }
                if j + 1 < grid[0].len() {
                    nsum += grid[i][j + 1];
                }
                sum[grid[i][j] as usize] = (nsum, dsum);
            }
        }
        Self { sum }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.sum[value as usize].0
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.sum[value as usize].1
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use itertools::multizip;
    #[test]
    fn test_neighbor_sum() {
        let null = 0;
        for test in [
            (
                vec![
                    "NeighborSum",
                    "adjacentSum",
                    "adjacentSum",
                    "diagonalSum",
                    "diagonalSum",
                ],
                vec_any![[[[0, 1, 2], [3, 4, 5], [6, 7, 8]]], [1], [4], [4], [8]],
                vec![null, 6, 16, 16, 4],
            ),
            (
                vec!["NeighborSum", "adjacentSum", "diagonalSum"],
                vec_any![
                    [[[1, 2, 0, 3], [4, 7, 15, 6], [8, 9, 10, 11], [12, 13, 14, 5]]],
                    [15],
                    [9]
                ],
                vec![null, 23, 45],
            ),
        ] {
            let obj = NeighborSum::new(test.1[0][0].as_vec_vec_i32());
            for (op, args, expect) in multizip(test).skip(1) {
                match op {
                    "adjacentSum" => assert_eq!(obj.adjacent_sum(args.as_slice()[0].as_i32()), expect),
                    "diagonalSum" => assert_eq!(obj.diagonal_sum(args.as_slice()[0].as_i32()), expect),
                    _ => panic!(),
                }
            }
        }
    }
}
