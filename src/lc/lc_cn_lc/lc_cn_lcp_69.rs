// https://leetcode.cn/problems/rMeRt2/
// LCP 69. Hello LeetCode!
pub struct Solution;
impl Solution {
    pub fn leetcode(words: Vec<String>) -> i32 {
        let s = "helloleetcode";
        let mut cnt = vec![0i32; 26];
        for &c in s.as_bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        let mut rule = vec![(0, 0, 0); 26];
        let mut nbit = 0;
        let mut full = 0;
        for (i, c) in cnt.into_iter().enumerate() {
            if c > 0 {
                let bitc = (i32::BITS - c.leading_zeros()) as i32;
                rule[i] = (nbit, c, (1 << bitc) - 1);
                full += c << nbit;
                nbit += bitc;
            }
        }
        let mut cost = Vec::with_capacity(words.len());
        fn find_cost(
            w: Vec<char>,
            rule: &[(i32, i32, i32)],
            mask: i32,
            costc: i32,
            costw: &mut std::collections::HashMap<i32, i32>,
        ) {
            if let Some(&cc) = costw.get(&mask) {
                if costc < cc {
                    costw.insert(mask, costc);
                }
            } else {
                costw.insert(mask, costc);
            }
            for (i, &c) in w.iter().enumerate() {
                let ic = (c as u8 - b'a') as usize;
                if rule[ic].2 == 0 {
                    continue;
                }
                let (bit, cnt, mr) = rule[ic];
                if (mask >> bit) & mr < cnt {
                    find_cost(
                        w[0..i].iter().chain(w[i + 1..w.len()].iter()).cloned().collect(),
                        rule,
                        mask + (1 << bit),
                        costc + (i * (w.len() - i - 1)) as i32,
                        costw,
                    );
                }
            }
        }
        for w in words {
            let mut cw = std::collections::HashMap::new();
            find_cost(w.chars().collect(), &rule, 0, 0, &mut cw);
            if cw.len() > 1 {
                cost.push(cw);
            }
        }
        let mut cache = vec![vec![-1; full as usize + 1]; cost.len()];
        rule = rule.into_iter().filter(|(_, _, m)| *m != 0).collect();
        fn dfs(
            i: usize,
            mask: usize,
            cost: &Vec<std::collections::HashMap<i32, i32>>,
            rule: &Vec<(i32, i32, i32)>,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if mask == 0 {
                return 0;
            }
            if i == cost.len() {
                return i32::MAX;
            }
            if cache[i][mask] >= 0 {
                return cache[i][mask];
            }
            let mut r = i32::MAX;
            'c: for (&m, &c) in &cost[i] {
                if c > r {
                    continue;
                }
                let mut nm = mask;
                for &(b, _, mm) in rule {
                    if ((nm as i32 >> b) & mm) < ((m >> b) & mm) {
                        continue 'c;
                    }
                    nm -= (m & (mm << b)) as usize;
                }
                r = r.min(dfs(i + 1, nm, cost, rule, cache).saturating_add(c));
            }
            cache[i][mask] = r;
            r
        }
        let r = dfs(0, full as usize, &cost, &rule, &mut cache);
        if r == i32::MAX {
            -1
        } else {
            r
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::leetcode(vec_str![
                "ee", "lubvvoem", "zwczl", "fxbhqxg", "ndmxwi", "lb", "saifswpo", "tcs", "pprkrtxk", "dhvkdrq", "lei",
                "sepoy", "ebawqho", "gcuwtn", "ngteo", "hfs", "ocbgncms", "kgtboqmf", "dotg", "jcvbbvy"
            ]),
            0
        );
        assert_eq!(Solution::leetcode(vec_str!["hold", "engineer", "cost", "level"]), 5);
        assert_eq!(Solution::leetcode(vec_str!["hello", "leetcode"]), 0);
    }
}
