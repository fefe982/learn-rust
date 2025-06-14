// https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer
// 1432. Max Difference You Can Get From Changing an Integer
pub struct Solution;
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut n = num;
        let mut large = 0;
        let mut small = 0;
        let mut small_to = 1;
        while n > 0 {
            small = n % 10;
            if small != 9 {
                large = small;
            }
            n /= 10;
        }
        if small == 1 {
            small_to = 0;
            small = 0;
            n = num;
            while n > 0 {
                let d = n % 10;
                if d != 0 && d != 1 {
                    small = d;
                }
                n /= 10;
            }
        }
        let mut t = 1;
        n = num;
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            if d == small {
                res += (d - small_to) * t;
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
    fn max_diff() {
        assert_eq!(Solution::max_diff(123456), 820000);
        assert_eq!(Solution::max_diff(555), 888);
        assert_eq!(Solution::max_diff(9), 8);
    }
}
