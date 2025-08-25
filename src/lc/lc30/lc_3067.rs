// https://leetcode.com/problems/count-pairs-of-connectable-servers-in-a-weighted-tree-network/
// 3067. Count Pairs of Connectable Servers in a Weighted Tree Network
pub struct Solution;
impl Solution {
    fn fill(
        g: &Vec<Vec<(usize, i32)>>,
        conn: &mut Vec<std::collections::HashMap<usize, Vec<i32>>>,
        p: usize,
        node: usize,
    ) -> Vec<i32> {
        let mut all = vec![];
        for &(n, w) in g[node].iter() {
            if n == p {
                continue;
            }
            if let Some(nvec) = conn[node].get(&n) {
                all.extend_from_slice(nvec);
                continue;
            }
            let mut nvec = Self::fill(g, conn, node, n);
            nvec.iter_mut().for_each(|ww| *ww += w);
            nvec.push(w);
            all.extend_from_slice(&nvec);
            conn[node].insert(n, nvec);
        }
        all
    }
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let mut g = vec![vec![]];
        for e in edges {
            let max = e[0].max(e[1]) as usize;
            if max >= g.len() {
                g.resize(max + 1, vec![]);
            }
            g[e[0] as usize].push((e[1] as usize, e[2]));
            g[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let n = g.len();
        let mut conn = vec![std::collections::HashMap::new(); n];
        for node in 0..n {
            Self::fill(&g, &mut conn, n, node);
        }
        let mut res = vec![0; n];
        for node in 0..n {
            let mut last = 0;
            for vec in conn[node].values() {
                let v = vec
                    .iter()
                    .filter_map(|w| if w % signal_speed == 0 { Some(w) } else { None })
                    .count() as i32;
                res[node] += last * v;
                last += v;
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
    fn test_count_pairs_of_connectable_servers() {
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(
                vec_vec![[0, 1, 1], [1, 2, 5], [2, 3, 13], [3, 4, 9], [4, 5, 2]],
                1
            ),
            [0, 4, 6, 6, 4, 0]
        );
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(
                vec_vec![[0, 6, 3], [6, 5, 3], [0, 3, 1], [3, 2, 7], [3, 1, 6], [3, 4, 2]],
                3
            ),
            [2, 0, 0, 0, 0, 0, 2]
        );
    }
}
