// https://leetcode.com/problems/solving-questions-with-brainpower/
// 2140. Solving Questions With Brainpower
pub struct Solution;
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let l = questions.len();
        let mut points = vec![0; l];
        points[l - 1] = questions[l - 1][0] as i64;
        for i in (0..l - 1).rev() {
            let mut p = questions[i][0] as i64;
            let n = i + questions[i][1] as usize + 1;
            if n < questions.len() {
                p += points[n];
            }
            points[i] = std::cmp::max(p, points[i + 1])
        }
        points[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn most_points() {
        assert_eq!(
            Solution::most_points(vec_vec![[3, 2], [4, 3], [4, 4], [2, 5]]),
            5
        );
        assert_eq!(
            Solution::most_points(vec_vec![[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]),
            7
        )
    }
}
