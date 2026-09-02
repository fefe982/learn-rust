// https://leetcode.com/problems/maximum-multiplication-score/
// 3290. Maximum Multiplication Score
pub struct Solution;
impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut max = [i64::MIN; 4];
        for i in 0..b.len() {
            for j in (1..a.len()).rev() {
                if j <= i {
                    max[j] = max[j].max(max[j - 1] + a[j] as i64 * b[i] as i64);
                }
            }
            max[0] = max[0].max(a[0] as i64 * b[i] as i64);
        }
        max[3]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7]), 26);
        assert_eq!(Solution::max_score(vec![-1, 4, 5, -2], vec![-5, -1, -3, -2, -4]), -1);
    }
}
