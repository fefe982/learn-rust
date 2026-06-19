// https://leetcode.com/problems/maximum-number-of-groups-entering-a-competition/
// 2358. Maximum Number of Groups Entering a Competition
pub struct Solution;
impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        let n = grades.len() as i32;
        let mut k = ((((8 * n) as f64).sqrt() - 1.0) / 2.0) as i32;
        if (k + 1) * (k + 2) / 2 <= n {
            k += 1;
        }
        k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_groups() {
        assert_eq!(Solution::maximum_groups(vec![10, 6, 12, 7, 3, 5]), 3);
        assert_eq!(Solution::maximum_groups(vec![8, 8]), 1);
    }
}
