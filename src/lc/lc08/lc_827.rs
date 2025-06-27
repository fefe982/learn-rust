// https://leetcode.com/problems/making-a-large-island/
// 827. Making A Large Island
pub struct Solution;
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut sz_map = std::collections::HashMap::new();
        let mut grid = grid;
        let mut max = 0;
        let mut idx = 2;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let mut sz = 0;
                    let mut q = vec![(i, j)];
                    grid[i][j] = idx;
                    while let Some((x, y)) = q.pop() {
                        sz += 1;
                        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            if nx >= n || ny >= n {
                                continue;
                            }
                            if grid[nx][ny] == 1 {
                                grid[nx][ny] = idx;
                                q.push((nx, ny));
                            }
                        }
                    }
                    sz_map.insert(idx, sz);
                    max = max.max(sz);
                    idx += 1;
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut set = std::collections::HashSet::<i32>::new();
                    let mut sz = 0;
                    for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let nx = (i as i32 + dx) as usize;
                        let ny = (j as i32 + dy) as usize;
                        if nx >= n || ny >= n {
                            continue;
                        }
                        if grid[nx][ny] > 0 && !set.contains(&grid[nx][ny]) {
                            set.insert(grid[nx][ny]);
                            sz += sz_map.get(&grid[nx][ny]).unwrap();
                        }
                    }
                    max = max.max(sz + 1);
                }
            }
        }

        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_largest_island() {
        assert_eq!(Solution::largest_island(vec_vec![[1, 0], [0, 1]]), 3);
        assert_eq!(Solution::largest_island(vec_vec![[1, 1], [0, 1]]), 4);
        assert_eq!(Solution::largest_island(vec_vec![[1, 1], [1, 1]]), 4);
    }
}
