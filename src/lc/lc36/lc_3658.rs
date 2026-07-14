// https://leetcode.com/problems/gcd-of-odd-and-even-sums/
// 3658. GCD of Odd and Even Sum
pub struct Solution;
impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_of_odd_even_sums() {
        assert_eq!(Solution::gcd_of_odd_even_sums(4), 4);
        assert_eq!(Solution::gcd_of_odd_even_sums(5), 5);
    }
}
