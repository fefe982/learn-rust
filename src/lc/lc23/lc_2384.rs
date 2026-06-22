// https://leetcode.com/problems/largest-palindromic-number/
// 2384. Largest Palindromic Number
pub struct Solution;
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut cnt = vec![0; 10];
        for c in num.as_bytes() {
            cnt[(c - b'0') as usize] += 1;
        }
        let mut c = 10;
        for i in (0..10).rev() {
            if c == 10 && cnt[i] % 2 == 1 {
                c = i;
            }
            cnt[i] /= 2;
        }
        let mut ans = String::new();
        for i in (1..10).rev() {
            for _ in 0..cnt[i] {
                ans.push((i as u8 + b'0') as char);
            }
        }
        let mut add_zero = false;
        if !ans.is_empty() {
            add_zero = true;
            for _ in 0..cnt[0] {
                ans.push('0');
            }
        }
        if c != 10 {
            ans.push((c as u8 + b'0') as char);
        }
        if add_zero {
            for _ in 0..cnt[0] {
                ans.push('0');
            }
        }
        for i in 1..10 {
            for _ in 0..cnt[i] {
                ans.push((i as u8 + b'0') as char);
            }
        }
        if ans.is_empty() && cnt[0] > 0 {
            ans.push('0');
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_palindromic() {
        assert_eq!(Solution::largest_palindromic("444947137".to_string()), "7449447");
        assert_eq!(Solution::largest_palindromic("00009".to_string()), "9");
    }
}
