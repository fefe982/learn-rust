// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/
// 3108. Minimum Cost Walk in Weighted Graph
pub struct Solution;
impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut v = (0..=n).collect::<Vec<_>>();
        let mut cost = vec![i32::MAX; n + 1];
        let get_p = |i: usize, v: &mut Vec<usize>| {
            let mut p = i;
            while p != v[p] {
                p = v[p];
            }
            let mut p1 = i;
            while p1 != p {
                let np = v[p1];
                v[p1] = p;
                p1 = np;
            }
            p
        };
        let mut merge = |i: usize, j: usize, c: i32, v: &mut Vec<usize>| {
            let pi = get_p(i, v);
            let pj = get_p(j, v);
            if v[i] != v[j] {
                v[pi] = pj;
                cost[pj] &= cost[pi] & c;
            } else {
                cost[pi] &= c;
            }
        };
        for e in edges {
            merge(e[0] as usize, e[1] as usize, e[2], &mut v);
        }
        let mut res = vec![-1; query.len()];
        for (i, q) in query.iter().enumerate() {
            let p1 = get_p(q[0] as usize, &mut v);
            let p2 = get_p(q[1] as usize, &mut v);
            if p1 == p2 {
                res[i] = cost[p1];
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_cost() {
        let mut e = vec![];
        let mut q = vec![];
        for i in 0..49999 {
            e.push(vec![i, i + 1, 1]);
            q.push(vec![i, i + 1]);
        }
        assert_eq!(Solution::minimum_cost(50000, e, q), vec![1; 49999]);
        assert_eq!(
            Solution::minimum_cost(5, vec_vec![[0, 1, 7], [1, 3, 7], [1, 2, 1]], vec_vec![[0, 3], [3, 4]]),
            [1, -1]
        );
        assert_eq!(
            Solution::minimum_cost(
                5,
                vec_vec![[0, 2, 7], [0, 1, 15], [1, 2, 6], [1, 2, 1]],
                vec_vec![[1, 2]]
            ),
            [0]
        );
    }
}
