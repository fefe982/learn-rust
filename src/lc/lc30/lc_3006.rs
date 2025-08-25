// https://leetcode.com/problems/find-beautiful-indices-in-the-given-array-i/
// 3006. Find Beautiful Indices in the Given Array I
pub struct Solution;
impl Solution {
    fn search(s: &[u8], a: &[u8]) -> Vec<i32> {
        let mut next = vec![0; a.len()];
        let mut i = 0;
        for j in 1..a.len() {
            while i > 0 && a[i] != a[j] {
                i = next[i - 1];
            }
            if a[i] == a[j] {
                i += 1;
            }
            next[j] = i;
        }
        i = 0;
        let mut res = vec![];
        for j in 0..s.len() {
            while i > 0 && a[i] != s[j] {
                i = next[i - 1];
            }
            if a[i] == s[j] {
                i += 1;
            }
            if i == a.len() {
                res.push((j + 1 - i) as i32);
                i = next[i - 1];
            }
        }
        res
    }
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let sa = Self::search(s.as_bytes(), a.as_bytes());
        let sb = Self::search(s.as_bytes(), b.as_bytes());
        let mut res = vec![];
        if sa.len() == 0 || sb.len() == 0 {
            return res;
        }
        let mut i = 0;
        let mut jl = 0;
        let mut jr = 0;
        while i < sa.len() && sa[i] + k < sb[jl] {
            i += 1;
        }
        while i < sa.len() {
            while jl + 1 < sb.len() && sb[jl + 1] <= sa[i] + k {
                jl += 1;
            }
            while jr < sb.len() && sb[jr] + k < sa[i] {
                jr += 1;
            }
            if jr >= sb.len() {
                break;
            }
            if jr <= jl {
                res.push(sa[i]);
            }
            i += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_beautiful_indices() {
        assert_eq!(
            Solution::beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".to_string(),
                "my".to_string(),
                "squirrel".to_string(),
                15
            ),
            [16, 33]
        );
        assert_eq!(
            Solution::beautiful_indices("abcd".to_string(), "a".to_string(), "a".to_string(), 4),
            [0]
        );
    }
}
