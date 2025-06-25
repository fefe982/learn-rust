// https://leetcode.com/problems/build-a-matrix-with-conditions/
// 2392. Build a Matrix With Conditions
pub struct Solution;
impl Solution {
    fn sort(k: usize, cond: Vec<Vec<i32>>) -> Vec<usize> {
        let mut before = vec![std::collections::HashSet::new(); k + 1];
        let mut after = vec![vec![]; k + 1];
        for c in cond {
            if before[c[1] as usize].insert(c[0] as usize) {
                after[c[0] as usize].push(c[1] as usize);
            }
        }
        let mut res = vec![];
        for i in 1..=k {
            if before[i].is_empty() {
                res.push(i);
            }
        }
        let mut idx = 0;
        while idx < res.len() {
            let cur = res[idx];
            for &i in &after[cur] {
                before[i].remove(&cur);
                if before[i].is_empty() {
                    res.push(i);
                }
            }
            idx += 1;
        }
        res
    }
    fn shuffle(sort: Vec<usize>) -> Vec<usize> {
        let mut res = vec![0; sort.len() + 1];
        for (i, n) in sort.into_iter().enumerate() {
            res[n] = i;
        }
        res
    }
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let k = k as usize;
        let rsort = Solution::sort(k, row_conditions);
        if rsort.len() < k {
            return vec![];
        }
        let csort = Solution::sort(k, col_conditions);
        if csort.len() < k {
            return vec![];
        }
        let rshuff = Solution::shuffle(rsort);
        let cshuff = Solution::shuffle(csort);
        let mut res = vec![vec![0; k]; k];
        for (n, (i, j)) in rshuff.into_iter().zip(cshuff.into_iter()).enumerate().skip(1) {
            res[i][j] = n as i32;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>, valid: bool) {
        let res = Solution::build_matrix(k, row_conditions.clone(), col_conditions.clone());
        if !valid {
            assert!(res.is_empty());
            return;
        }
        let k = k as usize;
        let mut pos = vec![(usize::MAX, usize::MAX); k + 1];
        let mut zcnt = 0;
        for i in 0..k {
            for j in 0..k {
                if res[i][j] == 0 {
                    zcnt += 1;
                } else {
                    assert!(pos[res[i][j] as usize].0 == usize::MAX);
                    pos[res[i][j] as usize] = (i, j);
                }
            }
        }
        assert_eq!(zcnt, k * k - k);
        for rcond in row_conditions {
            assert!(pos[rcond[0] as usize].0 < pos[rcond[1] as usize].0);
        }
        for ccond in col_conditions {
            assert!(pos[ccond[0] as usize].1 < pos[ccond[1] as usize].1);
        }
    }
    #[test]
    fn test_build_matrix() {
        check(
            8,
            vec_vec![
                [1, 2],
                [7, 3],
                [4, 3],
                [5, 8],
                [7, 8],
                [8, 2],
                [5, 8],
                [3, 2],
                [1, 3],
                [7, 6],
                [4, 3],
                [7, 4],
                [4, 8],
                [7, 3],
                [7, 5]
            ],
            vec_vec![[5, 7], [2, 7], [4, 3], [6, 7], [4, 3], [2, 3], [6, 2]],
            true,
        );
        check(3, vec_vec![[1, 2], [3, 2]], vec_vec![[2, 1], [3, 2]], true);
        check(3, vec_vec![[1, 2], [2, 3], [3, 1], [2, 3]], vec_vec![[2, 1]], false);
    }
}
