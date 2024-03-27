// https://leetcode.com/problems/strange-printer-ii/
// 1591. Strange Printer II
pub struct Solution;
impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let mut rng = std::collections::HashMap::<i32, (usize, usize, usize, usize)>::new();
        let m = target_grid.len();
        let n = target_grid[0].len();
        for i in 0..m {
            for j in 0..n {
                let v = target_grid[i][j];
                let r = rng.entry(v).or_insert((i, j, i, j));
                r.0 = r.0.min(i);
                r.1 = r.1.min(j);
                r.2 = r.2.max(i);
                r.3 = r.3.max(j);
            }
        }
        let mut edge_f = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        let mut edge_b = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        for (&c, (x1, y1, x2, y2)) in rng.iter() {
            for i in *x1..=*x2 {
                for j in *y1..=*y2 {
                    let v = target_grid[i][j];
                    if v != c {
                        edge_f.entry(v).or_default().insert(c);
                        edge_b.entry(c).or_default().insert(v);
                    }
                }
            }
        }
        let mut q = vec![];
        for &c in rng.keys() {
            if !edge_f.contains_key(&c) {
                q.push(c);
            }
        }
        while let Some(c) = q.pop() {
            if let Some(vc) = edge_b.remove(&c) {
                for v in vc {
                    edge_f.get_mut(&v).unwrap().remove(&c);
                    if edge_f[&v].is_empty() {
                        q.push(v);
                        edge_f.remove(&v);
                    }
                }
            }
        }
        edge_f.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_printable() {
        assert_eq!(
            Solution::is_printable(vec_vec![[1, 1, 1, 1], [1, 2, 2, 1], [1, 2, 2, 1], [1, 1, 1, 1]]),
            true
        );
        assert_eq!(
            Solution::is_printable(vec_vec![[1, 1, 1, 1], [1, 1, 3, 3], [1, 1, 3, 4], [5, 5, 1, 4]]),
            true
        );
        assert_eq!(Solution::is_printable(vec_vec![[1, 2, 1], [2, 1, 2], [1, 2, 1]]), false);
    }
}
