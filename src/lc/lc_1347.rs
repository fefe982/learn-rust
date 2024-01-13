// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
// 1347. Minimum Number of Steps to Make Two Strings Anagram
pub struct Solution;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut v = vec![0i32; 26];
        for c in s.bytes() {
            v[(c - b'a') as usize] += 1;
        }
        for c in t.bytes() {
            v[(c - b'a') as usize] -= 1;
        }
        v.into_iter().map(|x| x.abs()).sum::<i32>() / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_steps() {
        assert_eq!(Solution::min_steps(String::from("bab"), String::from("aba")), 1);
        assert_eq!(
            Solution::min_steps(String::from("leetcode"), String::from("practice")),
            5
        );
        assert_eq!(Solution::min_steps(String::from("anagram"), String::from("mangaar")), 0);
    }
}
