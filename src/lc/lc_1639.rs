// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/
// 1639. Number of Ways to Form a Target String Given a Dictionary
pub struct Solution;
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        if target.len() > words[0].len() {
            return 0;
        }
        let wl = words[0].len();
        let words = words.iter().fold(vec![[0; 26]; wl], |mut v, x| {
            x.as_bytes()
                .iter()
                .enumerate()
                .for_each(|(i, c)| v[i][(c - b'a') as usize] += 1);
            v
        });
        let target = target.as_bytes();
        let mut cnt = vec![0; target.len() + 1];
        cnt[0] = 1;
        let m = 1_000_000_007;
        for i in 0..wl {
            let mut ncnt = vec![0; target.len() + 1];
            ncnt[0] = 1;
            for j in 0..target.len() {
                ncnt[j + 1] = (cnt[j + 1] + (cnt[j] * words[i][(target[j] - b'a') as usize] as i64) % m) % m;
            }
            cnt = ncnt;
        }
        cnt[target.len()] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_ways() {
        assert_eq!(Solution::num_ways(vec_str!["abba", "baab"], String::from("babbbb")), 0);
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
        assert_eq!(Solution::num_ways(vec_str!["abba", "baab"], String::from("bab")), 4);
    }
}
