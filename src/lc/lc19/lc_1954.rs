// https://leetcode.com/problems/minimum-garden-perimeter-to-collect-enough-apples/
// 1954. Minimum Garden Perimeter to Collect Enough Apples
pub struct Solution;
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut min = 0i64;
        let mut max = 63000i64;
        while min + 1 < max {
            let mid = (min + max) / 2;
            if 2 * mid * (mid + 1) * (2 * mid + 1) >= needed_apples {
                max = mid;
            } else {
                min = mid;
            }
        }
        max * 8
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_perimeter() {
        assert_eq!(Solution::minimum_perimeter(1), 8);
        assert_eq!(Solution::minimum_perimeter(13), 16);
        assert_eq!(Solution::minimum_perimeter(1000000000), 5040);
    }
}
