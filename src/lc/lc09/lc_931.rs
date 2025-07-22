// https://leetcode.com/problems/minimum-falling-path-sum/
// 931. Minimum Falling Path Sum
pub struct Solution;
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let w = matrix[0].len();
        let mut save = vec![0; w];
        for l in matrix.iter() {
            let mut nsave = vec![0; w];
            for (i, &n) in l.iter().enumerate() {
                let mut m = save[i];
                let il = i.saturating_sub(1);
                let ir = i + 1;
                if il < w {
                    m = m.min(save[il]);
                }
                if ir < w {
                    m = m.min(save[ir]);
                }
                nsave[i] = n + m;
            }
            save = nsave;
        }
        save.into_iter().min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_falling_path_sum() {
        assert_eq!(
            Solution::min_falling_path_sum(vec_vec![[2, 1, 3], [6, 5, 4], [7, 8, 9]]),
            13
        );
        assert_eq!(
            Solution::min_falling_path_sum(vec_vec![[-19, 57], [-40, -5]]),
            -59
        );
    }
}
