// https://leetcode.com/problems/find-minimum-log-transportation-cost/
// 3560. Find Minimum Log Transportation Cost
pub struct Solution;
impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        if n > k {
            (n - k) as i64 * k as i64
        } else if m > k {
            (m - k) as i64 * k as i64
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cutting_cost() {
        assert_eq!(Solution::min_cutting_cost(6, 5, 5), 5);
        assert_eq!(Solution::min_cutting_cost(4, 4, 6), 0);
        assert_eq!(Solution::min_cutting_cost(4, 4, 6), 0);
    }
}
