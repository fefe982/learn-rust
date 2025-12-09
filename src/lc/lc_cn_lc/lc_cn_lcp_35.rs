// https://leetcode.cn/problems/DFPeFJ/
// LCP 35. 电动车游城市
pub struct Solution;
impl Solution {
    pub fn electric_car_plan(paths: Vec<Vec<i32>>, cnt: i32, start: i32, end: i32, charge: Vec<i32>) -> i32 {
        let mut g = vec![vec![]; 100];
        let mut n = 0;
        for p in paths {
            let (u, v, w) = (p[0] as usize, p[1] as usize, p[2]);
            g[u].push((v, w));
            g[v].push((u, w));
            n = n.max(u).max(v);
        }
        n += 1;
        let cnt = cnt as usize;
        let mut time = vec![vec![i32::MAX; cnt + 1]; n];
        let mut q = std::collections::BinaryHeap::new();
        let start = start as usize;
        let end = end as usize;
        q.push((std::cmp::Reverse(0), start, 0));
        while let Some((std::cmp::Reverse(t), u, c)) = q.pop() {
            if u == end {
                return t;
            }
            if time[u][c] <= t {
                continue;
            }
            time[u][c] = t;
            if c < cnt {
                q.push((std::cmp::Reverse(t + charge[u]), u, c + 1));
            }
            for &(v, w) in &g[u] {
                if c >= w as usize && t + w < time[v][c - w as usize] {
                    q.push((std::cmp::Reverse(t + w), v, c - w as usize));
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn electric_car_plan() {
        assert_eq!(
            Solution::electric_car_plan(
                vec_vec![[1, 3, 3], [3, 2, 1], [2, 1, 3], [0, 1, 4], [3, 0, 5]],
                6,
                1,
                0,
                vec![2, 10, 4, 1]
            ),
            43
        );
        assert_eq!(
            Solution::electric_car_plan(
                vec_vec![[0, 4, 2], [4, 3, 5], [3, 0, 5], [0, 1, 5], [3, 2, 4], [1, 2, 8]],
                8,
                0,
                2,
                vec![4, 1, 1, 3, 2]
            ),
            38
        );
    }
}
