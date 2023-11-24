// https://leetcode.com/problems/prefix-and-suffix-search/
// 745. Prefix and Suffix Search
pub struct WordFilter {
    m: std::collections::HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        let mut m = std::collections::HashMap::new();
        for (idx, w) in words.into_iter().enumerate() {
            for i in 1..=w.len() {
                for j in 0..w.len() {
                    let s = w
                        .chars()
                        .take(i)
                        .chain("|".chars())
                        .chain(w.chars().skip(j))
                        .collect::<String>();
                    m.insert(s, idx as i32);
                }
            }
        }
        Self { m }
    }

    pub fn f(&self, pref: String, suff: String) -> i32 {
        *self.m.get(&(pref + "|" + &suff)).unwrap_or(&-1)
    }
}
/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_filter() {
        let mut obj = WordFilter::new(vec!["apple".to_string()]);
        assert_eq!(obj.f("a".to_string(), "e".to_string()), 0);
    }
}
