// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/
// 1639. Number of Ways to Form a Target String Given a Dictionary
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    fn add(a: i64, b: i64) -> i64 {
        (a + b) % 1000000007
    }
    fn mul(a: i64, b: i64) -> i64 {
        (a * b) % 1000000007
    }
    fn count(
        words: &Vec<&[u8]>,
        target: &[u8],
        sw: usize,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if let Some(&cnt) = cache.get(&(sw, target.len())) {
            return cnt;
        }
        let mut cnt = 0;
        for w in words {
            if w[sw] == target[0] {
                cnt += 1;
            }
        }
        if target.len() > 1 {
            let mut cnt_rest = 0;
            for w in 0..(words[0].len() - sw - target.len() + 1) {
                cnt_rest = Self::add(
                    cnt_rest,
                    Self::count(words, &target[1..], sw + w + 1, cache),
                );
            }
            cnt = Self::mul(cnt, cnt_rest);
        }
        cache.insert((sw, target.len()), cnt);
        cnt
    }
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let words: Vec<&[u8]> = words.iter().map(|x| x.as_bytes()).collect();
        let target = target.as_bytes();
        let mut cnt = 0;
        let mut cache: HashMap<(usize, usize), i64> = HashMap::new();
        for start in 0..(words[0].len() - target.len() + 1) {
            cnt = Self::add(cnt, Self::count(&words, target, start, &mut cache));
        }
        cnt as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_ways() {
        assert_eq!(
            Solution::num_ways(
                vec_str![
                    "cabbaacaaaccaabbbbaccacbabbbcb",
                    "bbcabcbcccbcacbbbaacacaaabbbac",
                    "cbabcaacbcaaabbcbaabaababbacbc",
                    "aacabbbcaaccaabbaccacabccaacca",
                    "bbabbaabcaabccbbabccaaccbabcab",
                    "bcaccbbaaccaabcbabbacaccbbcbbb",
                    "cbbcbcaaaacacabbbabacbaabbabaa",
                    "cbbbbbbcccbabbacacacacccbbccca",
                    "bcbccbccacccacaababcbcbbacbbbc",
                    "ccacaabaaabbbacacbacbaaacbcaca",
                    "bacaaaabaabccbcbbaacacccabbbcb",
                    "bcbcbcabbccabacbcbcaccacbcaaab",
                    "babbbcccbbbbbaabbbacbbaabaabcc",
                    "baaaacaaacbbaacccababbaacccbcb",
                    "babbaaabaaccaabacbbbacbcbababa",
                    "cbacacbacaaacbaaaabacbbccccaca",
                    "bcbcaccaabacaacaaaccaabbcacaaa",
                    "cccbabccaabbcbccbbabaaacbacaaa",
                    "bbbcabacbbcabcbcaaccbcacacccca",
                    "ccccbbaababacbabcaacabaccbabaa",
                    "caaabccbcaaccabbcbcaacccbcacba",
                    "cccbcaacbabaacbaaabbbbcbbbbcbb",
                    "cababbcacbabcbaababcbcabbaabba",
                    "aaaacacaaccbacacbbbbccaabcccca",
                    "cbcaaaaabacbacaccbcbcbccaabaac",
                    "bcbbccbabaccabcccacbbaacbbcbba",
                    "cccbabbbcbbabccbbabbbbcaaccaab",
                    "acccacccaabbcaccbcaaccbababacc",
                    "bcacabaacccbbcbbacabbbbbcaaaab",
                    "cacccaacbcbccbabccabbcbabbcacc",
                    "aacabbabcaacbaaacaabcabcaccaab",
                    "cccacabacbabccbccaaaaabbcacbcc",
                    "cabaacacacaaabaacaabababccbaaa",
                    "caabaccaacccbaabcacbcbbabccabc",
                    "bcbbccbbaaacbaacbccbcbababcacb",
                    "bbabbcabcbbcababbbbccabaaccbca",
                    "cacbbbccabaaaaccacbcbabaabbcba",
                    "ccbcacbabababbbcbcabbcccaccbca",
                    "acccabcacbcbbcbccaccaacbabcaab",
                    "ccacaabcbbaabaaccbabcbacaaabaa",
                    "cbabbbbcabbbbcbccabaabccaccaca",
                    "acbbbbbccabacabcbbabcaacbbaacc",
                    "baaababbcabcacbbcbabacbcbaaabc",
                    "cabbcabcbbacaaaaacbcbbcacaccac"
                ],
                String::from("acbaccacbbaaabbbabac")
            ),
            555996654
        );
        assert_eq!(
            Solution::num_ways(vec_str!["acca", "bbbb", "caca"], String::from("aba")),
            6
        );
        assert_eq!(
            Solution::num_ways(vec_str!["abba", "baab"], String::from("bab")),
            4
        );
    }
}
