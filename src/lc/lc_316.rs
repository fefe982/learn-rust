// https://leetcode.com/problems/remove-duplicate-letters/
// 316. Remove Duplicate Letters
pub struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut last_pos = vec![0; 26];
        let s = s.as_bytes();
        s.iter()
            .enumerate()
            .for_each(|(i, &c)| last_pos[(c - b'a') as usize] = i);
        let mut p = vec![];
        let mut seen = vec![false; 26];
        for (i, &c) in s.iter().enumerate() {
            if seen[(c - b'a') as usize] {
                continue;
            }
            while let Some(&last) = p.last() {
                if last_pos[(last - b'a') as usize] > i && c < last {
                    p.pop();
                    seen[(last - b'a') as usize] = false;
                } else {
                    break;
                }
            }
            p.push(c);
            seen[(c - b'a') as usize] = true;
        }
        p.iter().map(|&c| c as char).collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb".to_string()
        );
    }
}
