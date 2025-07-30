// https://leetcode.com/problems/first-completely-painted-row-or-column/
// 2661. First Completely Painted Row or Column
pub struct Solution;
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut pos = vec![(usize::MAX, usize::MAX); m * n];
        let mut m_cnt = vec![0; m];
        let mut n_cnt = vec![0; n];
        let mut idx = 0;
        for i in 0..m {
            for j in 0..n {
                pos[mat[i][j] as usize - 1] = (i, j);
                loop {
                    let p = pos[arr[idx] as usize - 1];
                    if p.0 == usize::MAX {
                        break;
                    }
                    m_cnt[p.0] += 1;
                    n_cnt[p.1] += 1;
                    if m_cnt[p.0] == n || n_cnt[p.1] == m {
                        return idx as i32;
                    }
                    idx += 1;
                }
            }
        }
        idx as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_first_complete_index() {
        assert_eq!(
            Solution::first_complete_index(vec![1, 3, 4, 2], vec_vec![[1, 4], [2, 3]]),
            2
        );
        assert_eq!(
            Solution::first_complete_index(
                vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
                vec_vec![[3, 2, 5], [1, 4, 6], [8, 7, 9]]
            ),
            3
        );
    }
}
