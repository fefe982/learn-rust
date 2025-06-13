// https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/
// 2566. Maximum Difference by Remapping a Digit
pub struct Solution;
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let mut n = num;
        let mut large = 0;
        let mut small = 0;
        while n > 0 {
            small = n % 10;
            if small != 9 {
                large = small;
            }
            n /= 10;
        }
        let mut t = 1;
        n = num;
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            if d == small {
                res += d * t;
            }
            if d == large {
                res += (9 - d) * t;
            }
            n /= 10;
            t *= 10;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_max_difference() {
        assert_eq!(Solution::min_max_difference(11891), 99009);
        assert_eq!(Solution::min_max_difference(90), 99);
    }
}
