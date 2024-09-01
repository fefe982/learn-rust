// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
// 1450. Number of Students Doing Homework at a Given Time
pub struct Solution;
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .filter(|&(s, e)| s <= &query_time && e >= &query_time)
            .count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_busy_student() {
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
    }
}
