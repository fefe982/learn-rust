// https://leetcode.com/problems/valid-arrangement-of-pairs/
// 2097. Valid Arrangement of Pairs
pub struct Solution;
impl Solution {
    fn walk(m: &mut std::collections::HashMap<i32, Vec<i32>>, s: i32, ans: &mut Vec<Vec<i32>>) {
        while let Some(mut v) = m.remove(&s) {
            if let Some(next) = v.pop() {
                m.insert(s, v);
                Solution::walk(m, next, ans);
                ans.push(vec![s, next]);
            }
        }
    }
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut m = std::collections::HashMap::<i32, Vec<i32>>::new();
        let mut mm = std::collections::HashMap::<i32, i32>::new();
        for pair in pairs {
            m.entry(pair[0]).or_default().push(pair[1]);
            *mm.entry(pair[1]).or_default() += 1;
        }
        let mut s = *m.keys().next().unwrap();
        for (&k, v) in &m {
            if mm.get(&k).cloned().unwrap_or(0) < v.len() as i32 {
                s = k;
                break;
            }
        }
        let mut ans = vec![];
        Self::walk(&mut m, s, &mut ans);
        ans.reverse();
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(mut pairs: Vec<Vec<i32>>) {
        let mut ans = Solution::valid_arrangement(pairs.clone());
        assert_eq!(pairs.len(), ans.len());
        for i in 1..pairs.len() {
            assert_eq!(ans[i][0], ans[i - 1][1]);
        }
        pairs.sort_unstable();
        ans.sort_unstable();
        assert_eq!(pairs, ans);
    }
    #[test]
    fn test_valid_arrangement() {
        check(vec_vec![[5, 1], [4, 5], [11, 9], [9, 4]]);
        check(vec_vec![[1, 3], [3, 2], [2, 1]]);
        check(vec_vec![[1, 2], [1, 3], [2, 1]]);
    }
}
