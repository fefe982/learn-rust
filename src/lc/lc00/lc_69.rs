// https://leetcode.com/problems/sqrtx/
// 69. Sqrt(x)
pub struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        if x < 4 {
            return 1;
        }
        let mut min = 1;
        let mut max = 46340.min(x / 2);
        if max * max <= x {
            return max;
        }
        while min + 1 < max {
            let mid = (min + max) / 2;
            if mid * mid <= x {
                min = mid;
            } else {
                max = mid;
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
    }
}
