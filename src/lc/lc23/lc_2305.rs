// https://leetcode.com/problems/fair-distribution-of-cookies/
// 2305. Fair Distribution of Cookies
pub struct Solution;
impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut state = std::collections::HashSet::new();
        let mut s = std::collections::BTreeMap::new();
        s.insert(0, k);
        state.insert(s);
        for c in cookies {
            let mut nstate = std::collections::HashSet::new();
            for s in state {
                for (&v, &nv) in s.iter() {
                    let mut ns = s.clone();
                    if nv > 1 {
                        ns.insert(v, nv - 1);
                    } else {
                        ns.remove(&v);
                    }
                    *ns.entry(v + c).or_default() += 1;
                    nstate.insert(ns);
                }
            }
            state = nstate;
        }
        let mut min = i32::MAX;
        for s in state {
            min = min.min(*s.iter().rev().next().unwrap().0);
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distribute_cookies() {
        assert_eq!(Solution::distribute_cookies(vec![8, 15, 10, 20, 8], 2), 31);
        assert_eq!(
            Solution::distribute_cookies(vec![6, 1, 3, 2, 2, 4, 1, 2], 3),
            7
        );
    }
}
