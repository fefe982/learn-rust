// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
// 1337. The K Weakest Rows in a Matrix
pub struct Solution;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut rows = vec![];
        for (idx, row) in mat.into_iter().enumerate() {
            let mut count = 0;
            for val in row {
                if val == 0 {
                    break;
                }
                count += 1;
            }
            rows.push((count, idx));
        }
        rows.sort_by(|a, b| a.0.cmp(&b.0));
        rows.into_iter()
            .map(|(_, row)| row as i32)
            .take(k as usize)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_k_weakest_rows() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec_vec![
                    [1, 1, 0, 0, 0],
                    [1, 1, 1, 1, 0],
                    [1, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0],
                    [1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        );
        assert_eq!(
            Solution::k_weakest_rows(
                vec_vec![[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]],
                2
            ),
            vec![0, 2]
        );
    }
}
