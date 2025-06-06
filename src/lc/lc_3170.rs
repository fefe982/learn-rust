// https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/
// 3170. Lexicographically Minimum String After Removing Stars
pub struct Solution;
impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut v = s.chars().collect::<Vec<char>>();
        let mut m = 0i32;
        let mut stk = vec![vec![]; 26];
        for i in 0..v.len() {
            if v[i] == '*' {
                let p = m.trailing_zeros() as usize;
                v[stk[p].pop().unwrap()] = '*';
                if stk[p].is_empty() {
                    m ^= 1 << p;
                }
            } else {
                let p = v[i] as usize - 'a' as usize;
                stk[p].push(i);
                m |= 1 << p;
            }
        }
        v.into_iter().filter(|&c| c != '*').collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clear_stars() {
        assert_eq!(Solution::clear_stars("aaba*".to_string()), "aab".to_string());
        assert_eq!(Solution::clear_stars("abc".to_string()), "abc".to_string());
    }
}
