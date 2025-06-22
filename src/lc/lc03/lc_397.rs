// https://leetcode.com/problems/integer-replacement/
// 397. Integer Replacement
pub struct Solution;
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut c = 0;
        let mut n = n as u32;
        while n > 1 {
            if n < 4 {
                c += (n - 1) as i32;
                break;
            } else if n % 2 == 0 {
                let zc = n.trailing_zeros() as i32;
                n >>= zc;
                c += zc;
            } else if n & 2 == 2 {
                c += 1;
                n += 1;
            } else {
                c += 1;
                n -= 1;
            }
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_integer_replacement() {
        assert_eq!(Solution::integer_replacement(1234), 14);
        assert_eq!(Solution::integer_replacement(8), 3);
        assert_eq!(Solution::integer_replacement(7), 4);
        assert_eq!(Solution::integer_replacement(4), 2);
    }
}
