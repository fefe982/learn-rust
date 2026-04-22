// https://leetcode.com/problems/maximum-compatibility-score-sum/
// 1947. Maximum Compatibility Score Sum
pub struct Solution;
impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let mentor_count = mentors.len();
        let mut scores = vec![vec![0; mentor_count]; students.len()];
        for (student_index, student) in students.iter().enumerate() {
            for (mentor_index, mentor) in mentors.iter().enumerate() {
                scores[student_index][mentor_index] = student
                    .iter()
                    .zip(mentor.iter())
                    .filter(|(student_answer, mentor_answer)| student_answer == mentor_answer)
                    .count() as i32;
            }
        }

        let total_masks = 1usize << mentor_count;
        let mut dp = vec![0; total_masks];
        for mask in 0..total_masks {
            let student_index = mask.count_ones() as usize;
            if student_index == students.len() {
                continue;
            }
            for mentor_index in 0..mentor_count {
                if mask & (1usize << mentor_index) == 0 {
                    let next_mask = mask | (1usize << mentor_index);
                    dp[next_mask] = dp[next_mask].max(dp[mask] + scores[student_index][mentor_index]);
                }
            }
        }
        dp[total_masks - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_compatibility_sum() {
        assert_eq!(
            Solution::max_compatibility_sum(
                vec_vec![[1, 1, 0], [1, 0, 1], [0, 0, 1]],
                vec_vec![[1, 0, 0], [0, 0, 1], [1, 1, 0]]
            ),
            8
        );
        assert_eq!(
            Solution::max_compatibility_sum(vec_vec![[0, 0], [0, 0], [0, 0]], vec_vec![[1, 1], [1, 1], [1, 1]]),
            0
        );
    }
}
