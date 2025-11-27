// https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/
// 2894. Divisible and Non-Divisible Sums Difference
pub struct Solution;
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let c = n / m;
        (1 + n) * n / 2 - (1 + c) * c * m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn difference_of_sums() {
        assert_eq!(Solution::difference_of_sums(10, 3), 19);
        assert_eq!(Solution::difference_of_sums(5, 6), 15);
        assert_eq!(Solution::difference_of_sums(5, 1), -15);
    }
}
