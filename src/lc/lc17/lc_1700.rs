// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
// 1700. Number of Students Unable to Eat Lunch
pub struct Solution;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut s = students.into_iter().fold([0, 0], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });
        for sw in sandwiches {
            s[sw as usize] -= 1;
            if s[sw as usize] < 0 {
                return s[0] + s[1] + 1;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_students() {
        assert_eq!(Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
