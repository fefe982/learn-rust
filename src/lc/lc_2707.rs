// https://leetcode.com/problems/extra-characters-in-a-string/
// 2707. Extra Characters in a String
pub struct Solution;
struct Trie {
    leaf: bool,
    children: std::collections::HashMap<u8, Trie>,
}
impl Trie {
    pub fn insert(&mut self, s: &str) {
        self.insert_u8(s.as_bytes())
    }
    pub fn insert_u8(&mut self, s: &[u8]) {
        if s.len() == 0 {
            self.leaf = true;
        } else {
            self.children.entry(s[0]).or_default().insert_u8(&s[1..]);
        }
    }
    pub fn common_prefix_search(&self, s: &[u8]) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                res = child.common_prefix_search(&s[1..])
            }
        }
        for r in res.iter_mut() {
            *r += 1;
        }
        if self.leaf {
            res.push(0usize);
        }
        res
    }
}
impl Default for Trie {
    fn default() -> Self {
        Trie {
            leaf: false,
            children: std::collections::HashMap::new(),
        }
    }
}
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut word_trie = Trie::default();
        for word in dictionary {
            word_trie.insert(&word);
        }
        let mut subs = vec![i32::MAX; s.as_bytes().len() + 1];
        let s_u8 = s.as_bytes();
        let mut extra = 0;
        for i in 0..s_u8.len() {
            if extra < subs[i] {
                subs[i] = extra;
                extra += 1;
            } else {
                extra = subs[i] + 1;
                continue;
            }
            let mut pos = vec![i];
            while let Some(p) = pos.pop() {
                for n in word_trie.common_prefix_search(&s_u8[p..]) {
                    if subs[p] < subs[n + p] {
                        subs[n + p] = subs[p];
                        if n + p < s_u8.len() {
                            pos.push(n + p);
                        }
                    }
                }
            }
        }
        extra.min(subs[s_u8.len()])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_extra_char() {
        assert_eq!(
            Solution::min_extra_char(
                "kpqcavgyrvihakwwsa".to_string(),
                vec_str![
                    "xj", "yskd", "rva", "ft", "da", "dlh", "mncz", "sa", "dm", "ca", "ys", "mmo",
                    "o", "oaqf", "r", "ykj", "li", "jjn", "vf", "sx", "yz", "ej", "yrvi", "td",
                    "wws", "pt", "z"
                ]
            ),
            9
        );
        assert_eq!(
            Solution::min_extra_char(
                "kevlplxozaizdhxoimmraiakbak".to_string(),
                vec_str![
                    "yv", "bmab", "hv", "bnsll", "mra", "jjqf", "g", "aiyzi", "ip", "pfctr", "flr",
                    "ybbcl", "biu", "ke", "lpl", "iak", "pirua", "ilhqd", "zdhx", "fux", "xaw",
                    "pdfvt", "xf", "t", "wq", "r", "cgmud", "aokas", "xv", "jf", "cyys", "wcaz",
                    "rvegf", "ysg", "xo", "uwb", "lw", "okgk", "vbmi", "v", "mvo", "fxyx", "ad",
                    "e"
                ]
            ),
            9
        );
        assert_eq!(
            Solution::min_extra_char(
                "dwmodizxvvbosxxw".to_string(),
                vec_str![
                    "ox", "lb", "diz", "gu", "v", "ksv", "o", "nuq", "r", "txhe", "e", "wmo",
                    "cehy", "tskz", "ds", "kzbu"
                ]
            ),
            7
        );
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_string(),
                vec_str!["leet", "code", "leetcode"]
            ),
            1
        );
        assert_eq!(
            Solution::min_extra_char("sayhelloworld".to_string(), vec_str!["hello", "world"]),
            3
        );
    }
}
