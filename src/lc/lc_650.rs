// https://leetcode.com/problems/2-keys-keyboard/
// 650. 2 Keys Keyboard
pub struct Solution;
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut ans = 0;
        let mut n = n;
        let mut i = 2;
        while n >= i * i {
            while n % i == 0 {
                ans += i;
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            ans += n;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_steps() {
        assert_eq!(Solution::min_steps(3), 3);
        assert_eq!(Solution::min_steps(1), 0);
    }
}
