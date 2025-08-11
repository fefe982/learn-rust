// https://leetcode.com/problems/find-a-good-subset-of-the-matrix/
// 2732. Find a Good Subset of the Matrix
pub struct Solution;
impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let glen = grid.len();
        let nbit = grid[0].len();
        let mut midx = vec![usize::MAX; 1 << nbit];
        for i in 0..glen {
            let mut t = 0;
            for j in 0..nbit {
                if grid[i][j] == 1 {
                    t |= 1 << j;
                }
            }
            if t == 0 {
                return vec![i as i32];
            }
            midx[t] = i;
        }
        for i in 0..(1 << nbit) {
            if midx[i] == usize::MAX {
                continue;
            }
            for j in i + 1..(1 << nbit) {
                if midx[j] == usize::MAX {
                    continue;
                }
                if i & j == 0 {
                    if midx[i] < midx[j] {
                        return vec![midx[i] as i32, midx[j] as i32];
                    } else {
                        return vec![midx[j] as i32, midx[i] as i32];
                    }
                }
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(grid: Vec<Vec<i32>>, has_answer: bool) {
        let ans = Solution::good_subsetof_binary_matrix(grid.clone());
        if has_answer {
            assert!(!ans.is_empty());
            let sz = ans.len();
            let mut cnt = vec![0; grid[0].len()];
            for i in 0..ans.len() {
                if i > 0 {
                    assert!(ans[i] > ans[i - 1]);
                }
                for j in 0..grid[ans[i] as usize].len() {
                    cnt[j] += grid[ans[i] as usize][j];
                }
            }
            for i in 0..cnt.len() {
                assert!(cnt[i] <= sz as i32 / 2);
            }
        } else {
            assert!(ans.is_empty());
        }
    }
    #[test]
    fn test_good_subsetof_binary_matrix() {
        check(
            vec_vec![
                [1, 1, 1, 0, 0],
                [0, 1, 0, 1, 0],
                [1, 0, 0, 1, 0],
                [0, 1, 1, 1, 0],
                [1, 0, 1, 0, 0],
                [1, 0, 0, 1, 0],
                [0, 1, 1, 1, 1]
            ],
            true,
        );
        check(vec_vec![[0, 1, 1, 0], [0, 0, 0, 1], [1, 1, 1, 1]], true);
        check(vec_vec![[0]], true);
        check(vec_vec![[1, 1, 1], [1, 1, 1], [1, 1, 1]], false);
    }
}
