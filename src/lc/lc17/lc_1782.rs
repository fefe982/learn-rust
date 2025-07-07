// https://leetcode.com/problems/count-pairs-of-nodes/
// 1782. Count Pairs Of Nodes
pub struct Solution;
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut deg = vec![0i32; n as usize];
        let mut ec = std::collections::HashMap::<(usize, usize), i32>::new();
        for edge in edges {
            let e0 = edge[0] as usize - 1;
            let e1 = edge[1] as usize - 1;
            let e = if e0 < e1 { (e0, e1) } else { (e1, e0) };
            deg[e0] += 1;
            deg[e1] += 1;
            *ec.entry(e).or_default() += 1;
        }
        let mut degc = std::collections::HashMap::<i32, i32>::new();
        for &d in deg.iter() {
            *degc.entry(d).or_default() += 1;
        }
        let mdeg = *deg.iter().max().unwrap();
        let mut cnts = vec![0; mdeg as usize * 2 + 2];
        for (&deg1, &c1) in degc.iter() {
            for (&deg2, &c2) in degc.iter() {
                if deg1 < deg2 {
                    cnts[deg1 as usize + deg2 as usize] += c1 * c2;
                } else if deg1 == deg2 {
                    cnts[deg1 as usize + deg2 as usize] += c1 * (c1 - 1) / 2;
                }
            }
        }
        for ((e0, e1), c) in ec {
            if c == 0 {
                continue;
            }
            let s = (deg[e0] + deg[e1]) as usize;
            cnts[s] -= 1;
            cnts[s - c as usize] += 1;
        }
        for i in (1..cnts.len()).rev() {
            cnts[i - 1] += cnts[i];
        }
        queries
            .into_iter()
            .map(|q| cnts[(q as usize + 1).min(cnts.len() - 1)])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_pairs() {
        assert_eq!(
            Solution::count_pairs(
                5,
                vec_vec![[4, 5], [1, 3], [1, 4]],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 2]
            ),
            vec![10, 8, 10, 10, 8, 8, 10, 10, 10, 10, 8, 10, 10, 8, 10, 8, 8, 3]
        );
        assert_eq!(
            Solution::count_pairs(
                4,
                vec_vec![[1, 2], [2, 4], [1, 3], [2, 3], [2, 1]],
                vec![2, 3]
            ),
            vec![6, 5]
        );
        assert_eq!(
            Solution::count_pairs(
                5,
                vec_vec![
                    [1, 5],
                    [1, 5],
                    [3, 4],
                    [2, 5],
                    [1, 3],
                    [5, 1],
                    [2, 3],
                    [2, 5]
                ],
                vec![1, 2, 3, 4, 5]
            ),
            vec![10, 10, 9, 8, 6]
        );
    }
}
