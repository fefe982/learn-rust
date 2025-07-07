// https://leetcode.com/problems/average-waiting-time/
// 1701. Average Waiting Time
pub struct Solution;
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut sum = 0;
        let mut cur = 0;
        let n = customers.len();
        for c in customers {
            cur = cur.max(c[0]) + c[1];
            sum += (cur - c[0]) as i64;
        }
        sum as f64 / n as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_average_waiting_time() {
        assert_approx_eq!(Solution::average_waiting_time(vec_vec![[1, 2], [2, 5], [4, 3]]), 5.0);
        assert_approx_eq!(
            Solution::average_waiting_time(vec_vec![[5, 2], [5, 4], [10, 3], [20, 1]]),
            3.25
        );
    }
}
