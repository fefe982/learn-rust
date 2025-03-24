// https://leetcode.com/problems/difference-of-number-of-distinct-values-on-diagonals/
// 2711. Difference of Number of Distinct Values on Diagonals
pub struct Solution;
impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut res = vec![vec![0; n]; m];
        for l in 0..m + n - 1 {
            let (si, sj) = if l < m { (m - l - 1, 0) } else { (0, l - m + 1) };
            let (ei, ej) = if l < n { (m - 1, l) } else { (n + m - 2 - l, n - 1) };
            if si == ei {
                continue;
            }
            if si + 1 == ei {
                res[si][sj] = 1;
                res[ei][ej] = 1;
            }
            let mut m = std::collections::HashMap::<i32, i32>::new();
            let mut s = std::collections::HashSet::<i32>::new();
            for i in si..=ei {
                (*m.entry(grid[i][i - si + sj]).or_insert(0)) += 1;
            }
            for i in si..=ei {
                let d = grid[i][i - si + sj];
                let &c = m.get(&d).unwrap();
                if c == 1 {
                    m.remove(&d);
                } else {
                    m.insert(d, c - 1);
                }
                res[i][i - si + sj] = (s.len() as i32 - m.len() as i32).abs();
                s.insert(grid[i][i - si + sj]);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn difference_of_distinct_values() {
        assert_eq!(
            Solution::difference_of_distinct_values(vec_vec![[1, 2, 3], [3, 1, 5], [3, 2, 1]]),
            vec_vec![[1, 1, 0], [1, 0, 1], [0, 1, 1]]
        );
        assert_eq!(Solution::difference_of_distinct_values(vec_vec![[1]]), vec_vec![[0]]);
    }
}
