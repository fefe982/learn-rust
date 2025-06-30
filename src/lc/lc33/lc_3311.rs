// https://leetcode.com/problems/construct-2d-grid-matching-graph-layout/
// 3311. Construct 2D Grid to Match a 2D Array
pub struct Solution;
impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut r = vec![vec![]; 1];
        let mut min_rank = 5;
        let mut min_i = 0;
        let mut used = vec![false; n];
        for i in 0..g.len() {
            if g[i].len() < min_rank {
                min_rank = g[i].len();
                min_i = i;
            }
        }
        r[0].push(min_i as i32);
        used[min_i] = true;
        let mut q = std::collections::VecDeque::new();
        q.push_back((min_i, 0));
        used[min_i] = true;
        while let Some((c, row)) = q.pop_front() {
            let mut n = [usize::MAX, usize::MAX];
            let mut i = 0;
            for &nn in &g[c] {
                if !used[nn] {
                    n[i] = nn;
                    i += 1;
                }
            }
            if n[0] == usize::MAX {
                continue;
            }
            if n[1] == usize::MAX {
                if row + 1 == r.len() {
                    r.push(vec![]);
                }
                r[row + 1].push(n[0] as i32);
                q.push_back((n[0], row + 1));
                used[n[0]] = true;
            } else {
                let mut rt = 0;
                if g[n[0]].len() > g[n[1]].len() {
                    rt = 1;
                }
                r[row].push(n[rt] as i32);
                q.push_back((n[rt], row));
                if row + 1 == r.len() {
                    r.push(vec![]);
                }
                r[row + 1].push(n[1 - rt] as i32);
                q.push_back((n[1 - rt], row + 1));
                used[n[0]] = true;
                used[n[1]] = true;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(n: i32, edges: Vec<Vec<i32>>) {
        println!("check {n}, {edges:?}");
        let mut g = vec![vec![0; n as usize]; n as usize];
        for e in &edges {
            g[e[0] as usize][e[1] as usize] = 1;
            g[e[1] as usize][e[0] as usize] = 1;
        }
        let r = Solution::construct_grid_layout(n, edges);
        println!("r: {r:?}");
        let n = n as usize;
        assert!(r.len() * r[0].len() == n);
        let mut used = vec![false; n as usize];
        for i in 0..r.len() {
            assert!(r[i].len() == r[0].len());
            for j in 0..r[0].len() {
                if used[r[i][j] as usize] {
                    return;
                }
                assert!(!used[r[i][j] as usize]);
                used[r[i][j] as usize] = true;
                if i > 0 {
                    assert!(g[r[i][j] as usize][r[i - 1][j] as usize] == 1);
                }
                if j > 0 {
                    assert!(g[r[i][j] as usize][r[i][j - 1] as usize] == 1);
                }
                if i + 1 < r.len() {
                    assert!(g[r[i][j] as usize][r[i + 1][j] as usize] == 1);
                }
                if j + 1 < r[0].len() {
                    assert!(g[r[i][j] as usize][r[i][j + 1] as usize] == 1);
                }
            }
        }
    }
    #[test]
    fn test_construct_grid_layout() {
        check(4, vec_vec![[0, 1], [0, 2], [1, 3], [2, 3]]);
        check(5, vec_vec![[0, 1], [1, 3], [2, 3], [2, 4]]);
        check(
            9,
            vec_vec![
                [0, 1],
                [0, 4],
                [0, 5],
                [1, 7],
                [2, 3],
                [2, 4],
                [2, 5],
                [3, 6],
                [4, 6],
                [4, 7],
                [6, 8],
                [7, 8]
            ],
        );
    }
}
