// https://leetcode.com/problems/number-of-employees-who-met-the-target/
// 2798. Number of Employees Who Met the Target
pub struct Solution;
impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours
            .into_iter()
            .fold(0, |acc, x| if x >= target { acc + 1 } else { acc })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_employees_who_met_target() {
        assert_eq!(Solution::number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2), 3);
        assert_eq!(Solution::number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6), 0);
    }
}
