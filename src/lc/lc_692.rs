// https://leetcode.com/problems/top-k-frequent-words/
// 692. Top K Frequent Words
pub struct Solution;
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut m = std::collections::HashMap::new();
        for w in words {
            *m.entry(w).or_insert(0) += 1;
        }
        let mut v = m.into_iter().collect::<Vec<_>>();
        v.sort_by(|a, b| if a.1 == b.1 { a.0.cmp(&b.0) } else { b.1.cmp(&a.1) });
        v.into_iter().map(|(a, _)| a).take(k as usize).collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec_str!["i", "love", "leetcode", "i", "love", "coding"], 2),
            ["i", "love"]
        );
        assert_eq!(
            Solution::top_k_frequent(
                vec_str!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"],
                4
            ),
            ["the", "is", "sunny", "day"]
        );
    }
}
