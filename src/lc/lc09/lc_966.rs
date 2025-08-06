// https://leetcode.com/problems/vowel-spellchecker/
// 966. Vowel Spellchecker
pub struct Solution;
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut set = std::collections::HashSet::new();
        let mut mapi = std::collections::HashMap::new();
        let mut mapv = std::collections::HashMap::new();
        for word in wordlist {
            set.insert(word.clone());
            mapi.entry(word.to_lowercase()).or_insert(word.clone());
            let w: String = word
                .to_lowercase()
                .chars()
                .map(|c| match c {
                    'a' => '*',
                    'e' => '*',
                    'i' => '*',
                    'o' => '*',
                    'u' => '*',
                    _ => c,
                })
                .collect();
            mapv.entry(w).or_insert(word.clone());
        }
        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            if set.contains(&query) {
                ans.push(query);
            } else {
                let ql = query.to_lowercase();
                if let Some(v) = mapi.get(&ql) {
                    ans.push(v.clone());
                } else if let Some(v) = mapv.get(
                    &ql.chars()
                        .map(|c| match c {
                            'a' => '*',
                            'e' => '*',
                            'i' => '*',
                            'o' => '*',
                            'u' => '*',
                            _ => c,
                        })
                        .collect::<String>(),
                ) {
                    ans.push(v.clone());
                } else {
                    ans.push("".to_string());
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn spellchecker() {
        assert_eq!(
            Solution::spellchecker(
                vec_str!["KiTe", "kite", "hare", "Hare"],
                vec_str!["kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"]
            ),
            vec_str!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"]
        );
        assert_eq!(
            Solution::spellchecker(vec_str!["yellow"], vec_str!["YellOw"]),
            vec_str!["yellow"]
        );
    }
}
