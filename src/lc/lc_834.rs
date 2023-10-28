// https://leetcode.com/problems/sum-of-distances-in-tree/
// 834. Sum of Distances in Tree
pub struct Solution;
impl Solution {
    fn dist(
        from: i32,
        to: i32,
        t: &std::collections::HashMap<i32, std::collections::HashSet<i32>>,
        dp: &mut Vec<(i32, i32)>,
    ) -> (i32, i32) {
        let mut dist = 0;
        let mut cnt = 0;
        if let Some(out) = t.get(&to) {
            for &tt in out {
                if tt == from {
                    continue;
                }
                let (dtt, ctt) = Self::dist(to, tt, t, dp);
                dist += dtt;
                cnt += ctt;
            }
        }
        dist += cnt;
        cnt += 1;
        dp[to as usize] = (dist, cnt);
        (dist, cnt)
    }
    fn swp(dp: &mut Vec<(i32, i32)>, r: usize, c: usize) {
        dp[r] = (dp[r].0 - dp[c].0 - dp[c].1, dp[r].1 - dp[c].1);
        dp[c] = (dp[r].0 + dp[r].1 + dp[c].0, dp[r].1 + dp[c].1);
    }
    fn swap(
        from: i32,
        to: i32,
        t: &std::collections::HashMap<i32, std::collections::HashSet<i32>>,
        dp: &mut Vec<(i32, i32)>,
        res: &mut Vec<i32>,
    ) {
        if let Some(out) = t.get(&to) {
            for &tt in out {
                if tt == from {
                    continue;
                }
                Self::swp(dp, to as usize, tt as usize);
                res[tt as usize] = dp[tt as usize].0;
                Self::swap(to, tt, t, dp, res);
                Self::swp(dp, tt as usize, to as usize);
            }
        }
    }
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut t = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        for e in edges {
            t.entry(e[0]).or_default().insert(e[1]);
            t.entry(e[1]).or_default().insert(e[0]);
        }
        let mut res = vec![0; n as usize];
        let mut dp = vec![(0, 0); n as usize];
        Self::dist(-1, 0, &t, &mut dp);
        res[0] = dp[0].0;
        Self::swap(-1, 0, &t, &mut dp, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn sum_of_distances_in_tree() {
        assert_eq!(
            Solution::sum_of_distances_in_tree(6, vec_vec![[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]),
            vec![8, 12, 6, 10, 10, 10]
        );
        assert_eq!(Solution::sum_of_distances_in_tree(1, vec![]), vec![0]);
        assert_eq!(
            Solution::sum_of_distances_in_tree(2, vec_vec![[1, 0]]),
            vec![1, 1]
        );
    }
}
