// https://leetcode.com/problems/power-grid-maintenance/
// 3607. Power Grid Maintenance
pub struct Solution;
impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut ocnt = vec![0; c + 1];
        let mut nq = 0;
        for q in &queries {
            if q[0] == 2 {
                ocnt[q[1] as usize] += 1;
            } else {
                nq += 1;
            }
        }
        let mut grid = (0..=c)
            .into_iter()
            .map(|x| (x, if ocnt[x] > 0 { i32::MAX } else { x as i32 }))
            .collect::<Vec<_>>();
        fn get_p(grid: &mut Vec<(usize, i32)>, x: usize) -> usize {
            if grid[x].0 != x {
                grid[x].0 = get_p(grid, grid[x].0);
            }
            grid[x].0
        }
        fn merge(grid: &mut Vec<(usize, i32)>, x: usize, y: usize) {
            let px = get_p(grid, x);
            let py = get_p(grid, y);
            if px != py {
                grid[px].0 = py;
                grid[py].1 = grid[py].1.min(grid[px].1);
            }
        }
        for c in connections {
            merge(&mut grid, c[0] as usize, c[1] as usize);
        }
        let mut ans = vec![-1; nq];
        let mut iq = nq - 1;
        for q in queries.into_iter().rev() {
            let q1 = q[1] as usize;
            if q[0] == 2 {
                ocnt[q1] -= 1;
                if ocnt[q1] == 0 {
                    let pq = get_p(&mut grid, q1);
                    grid[pq].1 = grid[pq].1.min(q[1]);
                }
            } else {
                if ocnt[q1] == 0 {
                    ans[iq] = q[1];
                } else {
                    let pq = get_p(&mut grid, q1);
                    if grid[pq].1 < i32::MAX {
                        ans[iq] = grid[pq].1;
                    }
                }
                if iq == 0 {
                    break;
                }
                iq -= 1;
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
    fn process_queries() {
        assert_eq!(
            Solution::process_queries(
                5,
                vec_vec![[1, 2], [2, 3], [3, 4], [4, 5]],
                vec_vec![[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]],
            ),
            [3, 2, 3]
        );
        assert_eq!(
            Solution::process_queries(3, vec_vec![], vec_vec![[1, 1], [2, 1], [1, 1]],),
            [1, -1]
        );
    }
}
