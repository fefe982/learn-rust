// https://leetcode.com/problems/k-similar-strings/
// 854. K-Similar Strings
pub struct Solution;
impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        if s1 == s2 {
            return 0;
        }
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut q = std::collections::VecDeque::new();
        q.push_back((s1, 0));
        while let Some((s, i)) = q.pop_front() {
            for j in 0..s.len() {
                if s[j] == s2[j] {
                    continue;
                }
                let mut v = vec![false; 6];
                for k in j + 1..s.len() {
                    if s[k] == s2[j] && !v[s2[k] as usize - 'a' as usize] {
                        v[s2[k] as usize - 'a' as usize] = true;
                        let mut t = s.clone();
                        t.swap(k, j);
                        if t == s2 {
                            return i + 1;
                        }
                        q.push_back((t, i + 1));
                    }
                }
                break;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_k_similarity() {
        assert_eq!(
            Solution::k_similarity("baabaaabaabbbbbbbaba".to_string(), "abbabbbabbabaaababab".to_string()),
            9
        );
        assert_eq!(Solution::k_similarity("ab".to_string(), "ba".to_string()), 1);
        assert_eq!(Solution::k_similarity("abc".to_string(), "bca".to_string()), 2);
    }
}
