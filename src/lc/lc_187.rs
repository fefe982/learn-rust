// https://leetcode.com/problems/repeated-dna-sequences/
// 187. Repeated DNA Sequences
pub struct Solution;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut key = 0;
        let mut map = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let cc = match c {
                'A' => 0,
                'C' => 1,
                'G' => 2,
                'T' => 3,
                _ => unreachable!(),
            };
            key = ((key & ((1 << 18) - 1)) << 2) + cc;
            if i >= 9 {
                map.entry(key)
                    .and_modify(|(c, _)| *c += 1)
                    .or_insert((1, i));
            }
        }
        let mut res = vec![];
        for &(c, p) in map.values() {
            if c >= 2 {
                res.push(s[p - 9..p + 1].to_string());
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sort(mut s: Vec<String>) -> Vec<String> {
        s.sort();
        s
    }
    #[test]
    fn test_find_repeated_dna_sequences() {
        assert_eq!(
            sort(Solution::find_repeated_dna_sequences(
                "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()
            )),
            vec!["AAAAACCCCC", "CCCCCAAAAA"]
        );
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
            vec!["AAAAAAAAAA"]
        );
    }
}
