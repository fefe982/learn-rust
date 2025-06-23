// https://leetcode.com/problems/find-players-with-zero-or-one-losses/
// 2225. Find Players With Zero or One Losses
pub struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cnt = std::collections::BTreeMap::<i32, i32>::new();
        for m in matches {
            cnt.entry(m[0]).or_default();
            *cnt.entry(m[1]).or_default() += 1;
        }
        let mut ans = vec![vec![]; 2];
        for (k, v) in cnt {
            if v == 0 {
                ans[0].push(k);
            } else if v == 1 {
                ans[1].push(k);
            }
        }
        ans
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
