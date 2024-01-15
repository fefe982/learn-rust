// https://leetcode.com/problems/find-players-with-zero-or-one-losses/
// 2225. Find Players With Zero or One Losses
pub struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut z = std::collections::BTreeSet::new();
        let mut o = std::collections::BTreeSet::new();
        let mut t = std::collections::BTreeSet::new();
        for m in matches {
            if !t.contains(&m[0]) && !o.contains(&m[0]) {
                z.insert(m[0]);
            }
            if !t.contains(&m[1]) {
                if o.contains(&m[1]) {
                    o.remove(&m[1]);
                    t.insert(m[1]);
                } else {
                    if z.contains(&m[1]) {
                        z.remove(&m[1]);
                    }
                    o.insert(m[1]);
                }
            }
        }
        vec![z.into_iter().collect(), o.into_iter().collect()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_winners() {
        assert_eq!(
            Solution::find_winners(vec_vec![
                [1, 3],
                [2, 3],
                [3, 6],
                [5, 6],
                [5, 7],
                [4, 5],
                [4, 8],
                [4, 9],
                [10, 4],
                [10, 9]
            ]),
            vec_vec![[1, 2, 10], [4, 5, 7, 8]]
        );
        assert_eq!(
            Solution::find_winners(vec_vec![[2, 3], [1, 3], [5, 4], [6, 4]]),
            vec_vec![[1, 2, 5, 6], []]
        );
    }
}
