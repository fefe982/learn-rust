// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-i/
// 3014. Minimum Number of Pushes to Type Word I
pub struct Solution;
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut ans = 0;
        let mut n = word.len() as i32;
        while n > 0 {
            ans += n;
            n -= 8;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_pushes() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
        assert_eq!(Solution::minimum_pushes("xycdefghij".to_string()), 12);
    }
}
