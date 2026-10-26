// https://leetcode.com/problems/create-grid-with-exactly-one-path/
// 3963. Create Grid With Exactly One Path
pub struct Solution;
impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![];
        grid.push(std::iter::repeat('.').take(n).collect());
        for _ in 1..m {
            grid.push(std::iter::repeat('#').take(n - 1).chain(std::iter::once('.')).collect());
        }
        grid
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(m: i32, n: i32) {
        let grid = Solution::create_grid(m, n);
        let m = m as usize;
        let n = n as usize;
        let mut grid = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(grid.len(), m);
        for i in 0..m {
            assert_eq!(grid[i].len(), n);
            for j in 0..n {
                assert!(grid[i][j] == '.' || grid[i][j] == '#');
            }
        }
        assert_eq!(grid[0][0], '.');
        assert_eq!(grid[m - 1][n - 1], '.');
        let mut v: Vec<(usize, usize)> = vec![(0, 0)];
        grid[0][0] = '#';
        while let Some((i, j)) = v.pop() {
            for (ii, jj) in [(i + 1, j), (i.wrapping_sub(1), j), (i, j + 1), (i, j.wrapping_sub(1))] {
                if ii >= m || jj >= n {
                    continue;
                }
                if grid[ii][jj] == '.' {
                    grid[ii][jj] = '#';
                    v.push((ii, jj));
                }
            }
            assert!(v.len() <= 1);
        }
        assert_eq!(grid[m - 1][n - 1], '#');
    }
    #[test]
    fn create_grid() {
        check(2, 3);
        check(3, 3);
        check(1, 4);
    }
}
