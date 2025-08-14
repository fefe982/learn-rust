// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
// 1039. Minimum Score Triangulation of Polygon
pub struct Solution;
impl Solution {
    fn min_score_triangulation_ij(
        values: &Vec<i32>,
        i: usize,
        j: usize,
        cache: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if cache[i][j] == 0 {
            if j - i >= 2 {
                let mut min = i32::MAX;
                for k in i + 1..j {
                    min = std::cmp::min(
                        min,
                        Self::min_score_triangulation_ij(values, i, k, cache)
                            + values[i] * values[k] * values[j]
                            + Self::min_score_triangulation_ij(values, k, j, cache),
                    );
                }
                cache[i][j] = min;
            }
        }
        cache[i][j]
    }
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        Self::min_score_triangulation_ij(
            &values,
            0,
            values.len() - 1,
            &mut vec![vec![0; values.len()]; values.len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_score_triangulation() {
        assert_eq!(Solution::min_score_triangulation(vec![1, 2, 3]), 6);
        assert_eq!(
            Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5]),
            13
        );
    }
}
