// https://leetcode.com/problems/check-divisibility-by-digit-sum-and-product/
// 3622. Check Divisibility by Digit Sum and Product
pub struct Solution;
impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut nn = n;
        let mut sum = 0;
        let mut prod = 1;
        while nn > 0 {
            let d = nn % 10;
            sum += d;
            prod *= d;
            nn /= 10;
        }
        n % (sum + prod) == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_divisibility() {
        assert_eq!(Solution::check_divisibility(99), true);
        assert_eq!(Solution::check_divisibility(23), false);
    }
}
