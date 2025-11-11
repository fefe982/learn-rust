// https://leetcode.cn/problems/word-rectangle-lcci/
// 面试题 17.25. 单词矩阵
pub struct Solution;
use std::collections::BTreeMap;
use std::collections::HashMap;
struct TrieNode {
    leaf: bool,
    children: HashMap<u8, TrieNode>,
}
impl TrieNode {
    pub fn new() -> Self {
        Self {
            leaf: false,
            children: HashMap::new(),
        }
    }
    pub fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.leaf = true;
            return;
        }
        self.children
            .entry(word[0])
            .or_insert_with_key(|_| TrieNode::new())
            .insert(&word[1..]);
    }
}
impl Solution {
    fn check_line(
        words: &Vec<String>,
        left: usize,
        bottom: usize,
        right: usize,
        trie: Vec<&TrieNode>,
        semap: &HashMap<u8, HashMap<u8, Vec<usize>>>,
        line: usize,
        v: &mut Vec<usize>,
    ) -> bool {
        if line == words[left].len() - 1 {
            let wb = words[bottom].as_bytes();
            for i in 1..wb.len() - 1 {
                if let Some(t) = trie[i - 1].children.get(&wb[i]) {
                    if !t.leaf {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            v.push(bottom);
            return true;
        }
        if let Some(ll) = semap.get(&words[left].as_bytes()[line]) {
            if let Some(lr) = ll.get(&words[right].as_bytes()[line]) {
                for &wl in lr {
                    v.push(wl);
                    let mut ntrie = vec![];
                    let bwl = words[wl].as_bytes();
                    for i in 1..words[wl].len() - 1 {
                        if let Some(nn) = trie[i - 1].children.get(&bwl[i]) {
                            ntrie.push(nn);
                        } else {
                            break;
                        }
                    }
                    if ntrie.len() == trie.len() {
                        if Self::check_line(words, left, bottom, right, ntrie, semap, line + 1, v) {
                            return true;
                        }
                    }
                    v.pop();
                }
            }
        }
        false
    }
    fn check(
        words: &Vec<String>,
        top: usize,
        left: usize,
        bottom: usize,
        right: usize,
        trie: &TrieNode,
        semaph: &HashMap<u8, HashMap<u8, Vec<usize>>>,
        semapv: &HashMap<u8, HashMap<u8, Vec<usize>>>,
    ) -> Option<Vec<usize>> {
        let btop = words[top].as_bytes();
        let bbot = words[bottom].as_bytes();
        let bleft = words[left].as_bytes();
        let bright = words[right].as_bytes();
        for i in 1..btop.len() - 1 {
            if let Some(mapb) = semapv.get(&btop[i]) {
                if !mapb.contains_key(&bbot[i]) {
                    return None;
                }
            } else {
                return None;
            }
        }
        for i in 1..bleft.len() - 1 {
            if let Some(mapr) = semaph.get(&bleft[i]) {
                if !mapr.contains_key(&bright[i]) {
                    return None;
                }
            } else {
                return None;
            }
        }
        let mut v = vec![top];
        let mut vtrie = vec![];
        for i in 1..btop.len() - 1 {
            if let Some(nn) = trie.children.get(&words[top].as_bytes()[i]) {
                vtrie.push(nn);
            } else {
                return None;
            }
        }
        if Self::check_line(words, left, bottom, right, vtrie, semaph, 1, &mut v) {
            return Some(v);
        }
        None
    }
    pub fn max_rectangle(words: Vec<String>) -> Vec<String> {
        let mut semap = BTreeMap::new();
        let mut trie = TrieNode::new();
        let mut max_len = 0;
        let mut max_sz = 0;
        let mut ret = vec![];
        for (iw, word) in words.iter().enumerate() {
            let w = word.as_bytes();
            let len = w.len();
            max_len = max_len.max(len);
            semap
                .entry(len)
                .or_insert_with_key(|_| HashMap::new())
                .entry(w[0])
                .or_insert_with_key(|_| HashMap::new())
                .entry(w[len - 1])
                .or_insert_with_key(|_| vec![])
                .push(iw);
            trie.insert(w);
        }
        for (&toplen, mhorl) in semap.iter().rev() {
            for (&tl, mtopr) in mhorl {
                for (&tr, mtop) in mtopr {
                    for &itop in mtop.iter() {
                        for (&leftlen, mvert) in semap.iter().rev() {
                            if toplen * leftlen < max_sz {
                                break;
                            }
                            if let Some(mleftb) = mvert.get(&tl) {
                                if let Some(mrightb) = mvert.get(&tr) {
                                    for (&bl, mleft) in mleftb {
                                        for &ileft in mleft {
                                            if itop > ileft {
                                                continue;
                                            }
                                            if let Some(mbotr) = mhorl.get(&bl) {
                                                for (&br, mbot) in mbotr {
                                                    for &ibot in mbot {
                                                        if let Some(mright) = mrightb.get(&br) {
                                                            for &iright in mright {
                                                                if let Some(v) = Self::check(
                                                                    &words, itop, ileft, ibot, iright, &trie, mhorl,
                                                                    mvert,
                                                                ) {
                                                                    max_sz = toplen * leftlen;
                                                                    ret = v;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ret.into_iter().map(|x| words[x].clone()).collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(words: Vec<String>, expect: Vec<String>) {
        let mut set = std::collections::HashSet::new();
        for word in &words {
            set.insert(word.clone());
        }
        let res = Solution::max_rectangle(words);
        let sz = expect.len() * expect[0].len();
        if sz == 0 {
            assert!(res.len() == 0);
            return;
        } else {
            assert!(res.len() > 0);
        }
        assert!(sz == res.len() * res[0].len());
        let mut v = vec!["".to_string(); res[0].len()];
        for i in 0..res.len() {
            assert!(res[i].len() == res[0].len());
            assert!(set.contains(&res[i]));
            for (i, c) in res[i].chars().enumerate() {
                v[i].push(c);
            }
        }
        for word in v {
            assert!(set.contains(&word));
        }
    }
    #[test]
    fn test_max_rectangle() {
        check(
            vec_str!["this", "real", "hard", "trh", "hea", "iar", "sld"],
            vec_str!["this", "real", "hard"],
        );
        check(vec_str!["aa"], vec_str!["aa", "aa"]);
    }
}
