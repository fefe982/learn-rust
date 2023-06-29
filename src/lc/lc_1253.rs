// https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/
// 1253. Reconstruct a 2-Row Binary Matrix
pub struct Solution;
impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let (sum, mut dup) = colsum
            .iter()
            .fold((0, 0), |(s, f), &c| (s + c, f + if c == 2 { 1 } else { 0 }));
        if sum != upper + lower {
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
            Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]),
            vec_vec![[1, 1, 0], [0, 0, 1]]
        );
        assert_eq!(
            Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
            vec_vec![
                [1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
                [1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
            ]
        );
    }
}
