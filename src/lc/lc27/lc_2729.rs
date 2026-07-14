// https://leetcode.com/problems/check-if-the-number-is-fascinating/
// 2729. Check if the Number is Fascinating
pub struct Solution;
impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        if n > 333 {
            return false;
        }
        let s = format!("{}{}{}", n, n * 2, n * 3);
        let mut cnt = [0; 10];
        for c in s.chars() {
            cnt[c.to_digit(10).unwrap() as usize] += 1;
        }
        for i in 1..=9 {
            if cnt[i as usize] != 1 {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_fascinating() {
        assert_eq!(Solution::is_fascinating(192), true);
        assert_eq!(Solution::is_fascinating(100), false);
    }
}
