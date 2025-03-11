// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
// 1358. Number of Substrings Containing All Three Characters
pub struct Solution;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut cnt = vec![0; 3];
        let mut ans = 0;
        let mut left = 0;
        let s = s.as_bytes();
        for &c in s.iter() {
            cnt[c as usize - 'a' as usize] += 1;
            while cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 {
                cnt[s[left] as usize - 'a' as usize] -= 1;
                left += 1;
            }
            ans += left as i32;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_substrings() {
        assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
        assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
        assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
    }
}
