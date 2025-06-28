// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
// 1491. Average Salary Excluding the Minimum and Maximum Salary
pub struct Solution;
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut sum = 0;
        for &s in &salary {
            if s > max {
                max = s;
            }
            if s < min {
                min = s;
            }
            sum += s;
        }
        (sum - min - max) as f64 / (salary.len() - 2) as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn average() {
        assert_approx_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.0);
        assert_approx_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.0);
    }
}
