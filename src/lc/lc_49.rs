// https://leetcode.com/problems/group-anagrams/
// 49. Group Anagrams
pub struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut strs = strs
            .into_iter()
            .map(|s| {
                let mut v = s.chars().collect::<Vec<_>>();
                v.sort_unstable();
                (v, s)
            })
            .collect::<Vec<_>>();
        strs.sort_unstable();
        let mut last = strs[0].0.clone();
        let mut ans = vec![vec![strs[0].1.clone()]];
        for (v, s) in strs.into_iter().skip(1) {
            if v == last {
                ans.last_mut().unwrap().push(s);
            } else {
                ans.push(vec![s]);
            }
            last = v;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(strs: Vec<String>, ans: Vec<Vec<String>>, groups: i32) -> bool {
        if ans.len() as i32 != groups {
            return false;
        }
        let strs = strs.into_iter().collect::<std::collections::HashSet<_>>();
        let mut anss = std::collections::HashSet::new();
        for g in ans {
            if g.is_empty() {
                return false;
            }
            let mut v = g[0].chars().collect::<Vec<_>>();
            v.sort_unstable();
            for s in g {
                anss.insert(s.clone());
                let mut vs = s.chars().collect::<Vec<_>>();
                vs.sort_unstable();
                if vs != v || !strs.contains(&s) {
                    return false;
                }
            }
        }
        strs == anss
    }
    #[test]
    fn test_group_anagrams() {
        assert!(check(
            vec_str!["eat", "tea", "tan", "ate", "nat", "bat"],
            Solution::group_anagrams(vec_str!["eat", "tea", "tan", "ate", "nat", "bat"]),
            3
        ));
        assert!(check(vec_str![""], Solution::group_anagrams(vec_str![""]), 1));
        assert!(check(vec_str!["a"], Solution::group_anagrams(vec_str!["a"]), 1));
    }
}
