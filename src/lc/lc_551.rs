// https://leetcode.com/problems/student-attendance-record-i/
// 551. Student Attendance Record I
pub struct Solution;
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut late = 0;
        let mut absent = 0;
        for c in s.chars() {
            match c {
                'A' => {
                    absent += 1;
                    if absent > 1 {
                        return false;
                    }
                    late = 0;
                }
                'L' => {
                    late += 1;
                    if late > 2 {
                        return false;
                    }
                }
                _ => {
                    late = 0;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_record() {
        assert_eq!(Solution::check_record("LALL".to_string()), true);
        assert_eq!(Solution::check_record("PPALLP".to_string()), true);
        assert_eq!(Solution::check_record("PPALLL".to_string()), false);
    }
}
