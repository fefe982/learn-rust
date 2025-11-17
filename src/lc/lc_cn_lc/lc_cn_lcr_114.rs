// https://leetcode.cn/problems/Jf1JuT/
// LCR 114. 火星词典
pub struct Solution;
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph = std::collections::HashMap::new();
        let mut indegree = std::collections::HashMap::new();
        for word in &words {
            for c in word.chars() {
                graph.entry(c as u8).or_insert(std::collections::HashSet::new());
                indegree.entry(c as u8).or_insert(0);
            }
        }
        for i in 1..words.len() {
            let (w1, w2) = (&words[i - 1], &words[i]);
            let w1 = w1.as_bytes();
            let w2 = w2.as_bytes();
            let mut j = 0;
            while j < w1.len() && j < w2.len() {
                if w1[j] != w2[j] {
                    let c1 = w1[j];
                    let c2 = w2[j];
                    if graph.get_mut(&c1).unwrap().insert(c2) {
                        indegree.entry(c2).and_modify(|v| *v += 1);
                    }
                    break;
                }
                j += 1;
            }
            if j == w2.len() && j < w1.len() && (j == 0 || w1[j - 1] == w2[j - 1]) {
                return "".to_string();
            }
        }

        let mut queue = std::collections::VecDeque::new();
        for (&c, &indeg) in &indegree {
            if indeg == 0 {
                queue.push_back(c);
            }
        }
        let mut ans = String::new();
        while let Some(c) = queue.pop_front() {
            ans.push(c as char);
            for c2 in graph.get(&c).unwrap() {
                indegree.entry(*c2).and_modify(|v| *v -= 1);
                if indegree[c2] == 0 {
                    queue.push_back(*c2);
                }
            }
        }
        if ans.len() != indegree.len() {
            "".to_string()
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(words: Vec<String>, expect: &str) {
        let ans = Solution::alien_order(words.clone());
        assert!(ans.len() == expect.len());
        if expect.is_empty() {
            return;
        }
        let mut ord = vec![0; 26];
        for (i, c) in ans.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            assert!(ord[idx] == 0);
            ord[idx] = i + 1;
        }
        for i in 1..words.len() {
            let (w1, w2) = (&words[i - 1], &words[i]);
            let w1 = w1.as_bytes();
            let w2 = w2.as_bytes();
            let mut j = 0;
            while j < w1.len() && j < w2.len() {
                if w1[j] != w2[j] {
                    let c1 = w1[j];
                    let c2 = w2[j];
                    assert!(ord[(c1 - b'a') as usize] < ord[(c2 - b'a') as usize]);
                    break;
                }
                j += 1;
            }
        }
    }
    #[test]
    fn test() {
        check(vec_str!["ac", "ab", "zc", "zb"], "aczb");
        check(vec_str!["wrt", "wrf", "er", "ett", "rftt"], "wertf");
        check(vec_str!["z", "x"], "zx");
        check(vec_str!["z", "x", "z"], "");
    }
}
