// https://leetcode.com/problems/shortest-uncommon-substring-in-an-array/
// 3076. Shortest Uncommon Substring in an Array
pub struct Solution;
impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let mut cnt = std::collections::HashMap::new();
        for s in &arr {
            let ss = s.as_bytes();
            let mut set = std::collections::HashSet::new();
            for i in 0..ss.len() {
                for j in i + 1..=ss.len() {
                    let sub = &ss[i..j];
                    set.insert(sub);
                }
            }
            for sub in set {
                *cnt.entry(sub).or_insert(0) += 1;
            }
        }
        let mut ans = Vec::with_capacity(arr.len());
        for s in &arr {
            let ss = s.as_bytes();
            let mut mins = ss;
            let mut min_len = ss.len() + 1;
            for i in 0..ss.len() {
                for j in i + 1..=ss.len() {
                    let sub = &ss[i..j];
                    if cnt.get(sub) == Some(&1) {
                        if sub.len() < min_len || (sub.len() == min_len && sub < mins) {
                            min_len = min_len.min(sub.len());
                            mins = sub;
                        }
                    }
                }
            }
            if min_len == ss.len() + 1 {
                ans.push("".to_string());
            } else {
                ans.push(String::from_utf8_lossy(mins).to_string());
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
    fn shortest_substrings() {
        assert_eq!(
            Solution::shortest_substrings(vec_str!["cab", "ad", "bad", "c"]),
            vec_str!["ab", "", "ba", ""]
        );
        assert_eq!(
            Solution::shortest_substrings(vec_str!["abc", "bcd", "abcd"]),
            vec_str!["", "", "abcd"]
        );
    }
}
