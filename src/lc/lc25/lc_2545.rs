// https://leetcode.com/problems/sort-the-students-by-their-kth-score/
// 2545. Sort the Students by Their Kth Score
pub struct Solution;
impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score;
        score.sort_by(|a, b| b[k as usize].cmp(&a[k as usize]));
        score
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_sort_the_students() {
        assert_eq!(
            Solution::sort_the_students(vec_vec![[10, 6, 9, 1], [7, 5, 11, 2], [4, 8, 3, 15]], 2),
            [[7, 5, 11, 2], [10, 6, 9, 1], [4, 8, 3, 15]]
        );
        assert_eq!(
            Solution::sort_the_students(vec_vec![[3, 4], [5, 6]], 0),
            [[5, 6], [3, 4]]
        );
    }
}
