// https://leetcode.com/problems/interleaving-string/
// 97. Interleaving String
pub struct Solution;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut v = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        let mut q = vec![(0, 0)];
        while let Some((i1, i2)) = q.pop() {
            if v[i1][i2] {
                continue;
            }
            v[i1][i2] = true;
            if i1 < s1.len() && s1[i1] == s3[i1 + i2] {
                q.push((i1 + 1, i2));
            }
            if i2 < s2.len() && s2[i2] == s3[i1 + i2] {
                q.push((i1, i2 + 1));
            }
        }
        v[s1.len()][s2.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_interleave() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
        assert_eq!(
            Solution::is_interleave("".to_string(), "".to_string(), "".to_string()),
            true
        );
    }
}
