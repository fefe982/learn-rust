// https://leetcode.com/problems/lexicographically-smallest-string-after-deleting-duplicate-characters/
// 3816. Lexicographically Smallest String After Deleting Duplicate Characters
pub struct Solution;
impl Solution {
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let mut cnt = [0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut ans = Vec::new();
        for c in s.chars() {
            while let Some(&lc) = ans.last() {
                if cnt[(lc as u8 - b'a') as usize] > 1 && lc > c {
                    cnt[(lc as u8 - b'a') as usize] -= 1;
                    ans.pop();
                } else {
                    break;
                }
            }
            ans.push(c);
        }
        while let Some(&lc) = ans.last() {
            if cnt[(lc as u8 - b'a') as usize] > 1 {
                cnt[(lc as u8 - b'a') as usize] -= 1;
                ans.pop();
            } else {
                break;
            }
        }
        ans.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_smallest_after_deletion() {
        assert_eq!(Solution::lex_smallest_after_deletion("aaccb".to_string()), "aacb");
        assert_eq!(Solution::lex_smallest_after_deletion("z".to_string()), "z");
    }
}
