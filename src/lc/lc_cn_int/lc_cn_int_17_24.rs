// https://leetcode.cn/problems/max-submatrix-lcci/
// 面试题 17.24. Max Submatrix LCCI
pub struct Solution;
impl Solution {
    pub fn get_max_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0, 0, 0, 0];
        let mut sum = vec![0; matrix[0].len()];
        let mut max = i32::MIN;
        for i in 0..matrix.len() {
            sum.fill(0);
            for j in i..matrix.len() {
                let mut s = 0;
                let mut is = 0;
                for k in 0..matrix[0].len() {
                    sum[k] += matrix[j][k];
                    if s <= 0 {
                        is = k;
                        s = sum[k];
                    } else {
                        s += sum[k];
                    }
                    if s > max {
                        max = s;
                        ans = vec![i as i32, is as i32, j as i32, k as i32];
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(matrix: Vec<Vec<i32>>, expect: Vec<i32>) {
        fn get_sum(matrix: &Vec<Vec<i32>>, bound: &Vec<i32>) -> i32 {
            let mut sum = 0;
            for i in bound[0] as usize..=bound[2] as usize {
                for j in bound[1] as usize..=bound[3] as usize {
                    sum += matrix[i][j];
                }
            }
            sum
        }
        let ans = Solution::get_max_matrix(matrix.clone());
        assert_eq!(get_sum(&matrix, &expect), get_sum(&matrix, &ans))
    }
    #[test]
    fn get_max_matrix() {
        check(vec_vec![[-1, 0], [0, -1]], vec![0, 1, 0, 1]);
    }
}
