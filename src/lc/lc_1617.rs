// https://leetcode.com/problems/count-subtrees-with-max-distance-between-cities/
// 1617. Count Subtrees With Max Distance Between Cities
pub struct Solution;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
impl Solution {
    fn add_edge(
        node_rank: &mut HashMap<i32, HashSet<i32>>,
        dist: &mut Vec<Vec<i32>>,
        i: i32,
        j: i32,
    ) {
        Self::add_edge_d(node_rank, dist, i, j);
        Self::add_edge_d(node_rank, dist, j, i);
    }

    fn add_edge_d(
        node_rank: &mut HashMap<i32, HashSet<i32>>,
        dist: &mut Vec<Vec<i32>>,
        i: i32,
        j: i32,
    ) {
        if let Some(v) = node_rank.get_mut(&i) {
            v.insert(j);
        } else {
            node_rank.insert(i, HashSet::from([j]));
        }
        dist[i as usize][j as usize] = 1;
    }
    fn count_dist(
        n: i32,
        node_rank: &mut HashMap<i32, HashSet<i32>>,
        dist_cnt: &mut BTreeMap<i32, i32>,
        dist: &Vec<Vec<i32>>,
        subtree_cnt: &mut Vec<i32>,
        cache: &mut BTreeSet<BTreeSet<i32>>,
        extracted: &mut BTreeSet<i32>,
    ) {
        for (d, &c) in dist_cnt.iter().rev() {
            if c > 0 {
                subtree_cnt[(*d - 1) as usize] += 1;
                break;
            }
        }
        cache.insert(extracted.clone());
        if extracted.len() + 2 == n as usize {
            return;
        }
        for inode in 0..n {
            if node_rank[&inode].len() == 1 {
                let jnode = *node_rank[&inode].iter().next().unwrap();
                extracted.insert(inode);
                if cache.contains(&extracted) {
                    extracted.remove(&inode);
                    continue;
                }
                node_rank.get_mut(&inode).unwrap().remove(&jnode);
                node_rank.get_mut(&jnode).unwrap().remove(&inode);
                for cnode in 0..n {
                    if !extracted.contains(&cnode) && dist[inode as usize][cnode as usize] != 0 {
                        *dist_cnt
                            .get_mut(&dist[inode as usize][cnode as usize])
                            .unwrap() -= 1;
                    }
                }
                Self::count_dist(n, node_rank, dist_cnt, dist, subtree_cnt, cache, extracted);
                for cnode in 0..n {
                    if !extracted.contains(&cnode) && dist[inode as usize][cnode as usize] != 0 {
                        *dist_cnt
                            .get_mut(&dist[inode as usize][cnode as usize])
                            .unwrap() += 1;
                    }
                }
                node_rank.get_mut(&inode).unwrap().insert(jnode);
                node_rank.get_mut(&jnode).unwrap().insert(inode);
                extracted.remove(&inode);
            }
        }
    }
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut node_rank = HashMap::new();
        let mut dist_cnt: BTreeMap<i32, i32> = BTreeMap::new();
        let mut dist = vec![vec![0; n as usize]; n as usize];
        let mut subtree_cnt = vec![0; (n - 1) as usize];
        for v in edges.iter() {
            Self::add_edge(&mut node_rank, &mut dist, v[0] - 1, v[1] - 1);
        }
        for i in 0..(n as usize) {
            for j in 0..((n - 1) as usize) {
                if i == j {
                    continue;
                }
                for k in (j + 1)..(n as usize) {
                    if i == k {
                        continue;
                    }
                    if dist[i][j] != 0
                        && dist[i][k] != 0
                        && (dist[j][k] == 0 || dist[i][j] + dist[i][k] < dist[j][k])
                    {
                        let d = dist[i][j] + dist[i][k];
                        dist[j][k] = d;
                        dist[k][j] = d;
                    }
                }
            }
        }
        for j in 0..((n - 1) as usize) {
            for k in (j + 1)..(n as usize) {
                if let Some(c) = dist_cnt.get_mut(&dist[j][k]) {
                    *c += 1;
                } else {
                    dist_cnt.insert(dist[j][k], 1);
                }
            }
        }
        Self::count_dist(
            n,
            &mut node_rank,
            &mut dist_cnt,
            &dist,
            &mut subtree_cnt,
            &mut BTreeSet::new(),
            &mut BTreeSet::new(),
        );
        subtree_cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_subgraphs_for_each_diameter() {
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(
                4,
                vec![vec![1, 2], vec![2, 3], vec![2, 4]]
            ),
            vec![3, 4, 0]
        );
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(2, vec![vec![1, 2]]),
            vec![1]
        );
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(3, vec![vec![1, 2], vec![2, 3]]),
            vec![2, 1]
        );
    }
}
