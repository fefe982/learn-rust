// https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/
// 1253. Reconstruct a 2-Row Binary Matrix
pub struct Solution;
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut upper = upper;
        let mut lower = lower;
        let (sum, cnt, mut dup) = colsum.iter().fold((0, 0, 0), |(s, cn, f), &c| {
            (s + c, cn + if c != 0 { 1 } else { 0 }, f + if c == 2 { 1 } else { 0 })
        });
        if sum != upper + lower || cnt < upper || cnt < lower {
            return vec![];
        }
        let mut res = vec![vec![0; colsum.len()]; 2];
        for (idx, csum) in colsum.into_iter().enumerate() {
            if csum == 1 {
                if upper > dup {
                    res[0][idx] = 1;
                    upper -= 1;
                } else {
                    res[1][idx] = 1;
                    lower -= 1;
                }
            } else if csum == 2 {
                res[0][idx] = 1;
                res[1][idx] = 1;
                dup -= 1;
                upper -= 1;
                lower -= 1;
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
    fn reconstruct_matrix() {
        assert_eq!(
            Solution::reconstruct_matrix(9, 2, vec![0, 1, 2, 0, 0, 0, 0, 0, 2, 1, 2, 1, 2]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]),
            vec_vec![[1, 1, 0], [0, 0, 1]]
        );
        assert_eq!(
            Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
            vec_vec![[1, 1, 1, 0, 1, 0, 0, 1, 0, 0], [1, 0, 1, 0, 0, 0, 1, 1, 0, 1]]
        );
    }
}
